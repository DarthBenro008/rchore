use crate::{
    models::{tasklist::TaskList, tasks::Tasks},
    service::{
        database_api::TasksDatabase, google_api::GoogleApiClient, google_tasklist::ApiTaskList,
        google_tasks::ApiTasks,
    },
};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};
use std::io::stdout;

struct State {
    google_api_client: GoogleApiClient,
    should_quit: bool,
    tasklists: Vec<TaskList>,
    tasks: Vec<Tasks>,
    selected_tasklist: usize,
    selected_task: usize,
    show_error: Option<String>,
    is_hidden_shown: bool,
    create_task: Option<String>, // TODO https://github.com/sayanarijit/tui-input/blob/main/examples/ratatui-input/src/main.rs
}

pub fn open(tasks_database: TasksDatabase) -> anyhow::Result<()> {
    let google_api_client = GoogleApiClient::new(tasks_database);

    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut state = State {
        google_api_client,
        should_quit: false,
        tasklists: Vec::new(),
        tasks: Vec::new(),
        selected_tasklist: 0,
        selected_task: 0,
        show_error: None,
        is_hidden_shown: false,
        create_task: None,
    };

    fetch_tasklists(&mut state);
    fetch_tasks(&mut state);

    while !state.should_quit {
        update(&mut state)?;
        terminal.draw(|frame| {
            ui(frame, &state);
        })?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn update(state: &mut State) -> anyhow::Result<()> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                if key.code == KeyCode::Char('q') {
                    state.should_quit = true;
                }
                let value_ro = state.create_task.clone().unwrap_or("".to_string());
                if let Some(value) = &mut state.create_task {
                    match key.code {
                        KeyCode::Backspace => {
                            value.pop();
                        }
                        KeyCode::Delete => todo!(),
                        KeyCode::Enter => {
                            create_task(state, value_ro);
                            state.create_task = None;
                        }
                        KeyCode::Left => todo!(),
                        KeyCode::Right => todo!(),
                        KeyCode::Tab => todo!(),
                        KeyCode::Char(c) => {
                            value.push(c);
                        }
                        KeyCode::Esc => {
                            state.create_task = None;
                        }
                        _ => (),
                    }
                } else {
                    if key.code == KeyCode::Char('s') {
                        state.is_hidden_shown = !state.is_hidden_shown;
                        fetch_tasks(state);
                    }

                    if key.code == KeyCode::Char('c') {
                        state.create_task = Some("".to_string());
                    }

                    if key.code == KeyCode::Char('l') {
                        state.selected_tasklist =
                            if state.selected_tasklist == state.tasklists.len() - 1 {
                                0
                            } else {
                                state.selected_tasklist + 1
                            };
                        state.selected_task = 0;
                        state.google_api_client.tasklist =
                            state.tasklists[state.selected_tasklist].id.clone();
                        fetch_tasks(state);
                    }

                    if key.code == KeyCode::Char('h') {
                        state.selected_tasklist = if state.selected_tasklist == 0 {
                            state.tasklists.len() - 1
                        } else {
                            state.selected_tasklist - 1
                        };
                        state.selected_task = 0;
                        state.google_api_client.tasklist =
                            state.tasklists[state.selected_tasklist].id.clone();
                        fetch_tasks(state);
                    }

                    if key.code == KeyCode::Char('j') {
                        state.selected_task = if state.selected_task == state.tasks.len() - 1 {
                            0
                        } else {
                            state.selected_task + 1
                        };
                    }

                    if key.code == KeyCode::Char('k') {
                        state.selected_task = if state.selected_task == 0 {
                            state.tasks.len() - 1
                        } else {
                            state.selected_task - 1
                        };
                    }

                    if key.code == KeyCode::Char(' ') {
                        toggle_taks_completed(state);
                    }
                }
            }
        }
    }

    Ok(())
}

fn fetch_tasklists(state: &mut State) {
    let resp = state.google_api_client.fetch_tasklist(false);
    match resp {
        Ok(data) => state.tasklists = data.items.clone(),
        Err(_err) => state.show_error = Some("Can not fetch taskslists".to_string()),
    };
}

fn fetch_tasks(state: &mut State) {
    let resp = state
        .google_api_client
        .fetch_all_tasks(state.is_hidden_shown);
    match resp {
        Ok(data) => state.tasks = data.items.clone(),
        Err(_err) => state.show_error = Some("Can not fetch tasks".to_string()),
    };
}

fn toggle_taks_completed(state: &mut State) {
    if let Some(task) = state.tasks.get_mut(state.selected_task) {
        task.status = if task.status == "completed" {
            "needsAction"
        } else {
            "completed"
        }
        .to_string();

        let resp = state.google_api_client.update_task(task.clone());
        match resp {
            Ok(data) => *task = data.clone(),
            Err(_err) => state.show_error = Some("Can not update tasks".to_string()),
        }
    }
}

fn create_task(state: &mut State, title: String) {
    let task = Tasks::new(None, title, String::from(""), String::from("needsAction"));

    let resp = state.google_api_client.add_task(task);
    match resp {
        Ok(data) => state.tasks.push(data),
        Err(_err) => state.show_error = Some("Can not fetch tasks".to_string()),
    };
}

fn ui(frame: &mut Frame, state: &State) {
    let main_layout = Layout::new(
        Direction::Vertical,
        [Constraint::Min(0), Constraint::Length(1)],
    )
    .split(frame.size());

    {
        let area = Layout::new(
            Direction::Vertical,
            [
                Constraint::Length(1),
                Constraint::Min(0),
                Constraint::Length(8),
            ],
        )
        .split(main_layout[0]);

        {
            let tasklist_titles = state
                .tasklists
                .iter()
                .map(|task_list| format!(" {} ", task_list.title.clone()))
                .collect::<Vec<String>>();

            if tasklist_titles.is_empty() {
                frame.render_widget(Text::raw("You got no tasklists"), area[0]);
            } else {
                let tabs = Tabs::new(tasklist_titles)
                    .style(Style::default().white())
                    .highlight_style(Style::default().white().bg(Color::Blue))
                    .divider("⣿")
                    .padding("", "")
                    .select(state.selected_tasklist);

                frame.render_widget(tabs, area[0]);
            }

            if state.tasks.is_empty() {
                frame.render_widget(
                    Paragraph::new("There are no Tasks yet")
                        .block(Block::new().padding(Padding::uniform(1))),
                    area[1],
                );
            } else {
                let items = state.tasks.iter().map(|task| view_task(task.clone()));

                let mut list_state = ListState::default().with_selected(Some(state.selected_task));

                let list = List::new(items)
                    .block(Block::default().title("List").borders(Borders::ALL))
                    .style(Style::default().fg(Color::White))
                    .highlight_style(Style::default().bg(Color::Blue).white())
                    .block(
                        Block::default()
                            .title("Tasks")
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded),
                    );
                frame.render_stateful_widget(list, area[1], &mut list_state);

                if let Some(task) = state.tasks.get(state.selected_task) {
                    frame.render_widget(
                        Paragraph::new(task.notes.to_owned()).block(
                            Block::new()
                                .title("Notes")
                                .borders(Borders::ALL)
                                .border_type(BorderType::Rounded),
                        ),
                        area[2],
                    );
                } else {
                    frame.render_widget(
                        Paragraph::new("No task selected".to_string()).block(
                            Block::new()
                                .title("Notes")
                                .borders(Borders::ALL)
                                .border_type(BorderType::Rounded),
                        ),
                        area[2],
                    );
                }
            }
        }

        // TODO replace with better string concat
        // let actions = possible_actions()
        //     .iter()
        //     .filter(|a| {
        //         let f = &*a.func;
        //         f(state.clone()).is_some()
        //     })
        //     .map(
        //         |Action {
        //              key_code,
        //              name,
        //              func: _,
        //          }| {
        //             let key = key_code_to_string(key_code);
        //             format!("{key}: {name}")
        //         },
        //     )
        //     .collect::<Vec<String>>()
        //     .join(", ");

        let actions = state
            .google_api_client
            .tasklist
            .clone()
            .unwrap_or(String::from("no tasklist"));

        frame.render_widget(Paragraph::new(actions), main_layout[1]);

        if let Some(value) = state.create_task.clone() {
            let layout = Layout::new(
                Direction::Vertical,
                [
                    Constraint::Fill(1),
                    Constraint::Length(3),
                    Constraint::Fill(1),
                ],
            )
            .margin(4)
            .split(frame.size());

            frame.render_widget(Clear, layout[1]);
            frame.render_widget(
                Paragraph::new(Line::from(vec![Span::from(value), " ".slow_blink()])).block(
                    Block::new()
                        .title("create task")
                        .borders(Borders::all())
                        .border_type(BorderType::Rounded),
                ),
                layout[1],
            )
        }
    }
}

fn view_task<'a>(task: Tasks) -> ListItem<'a> {
    let is_completed = task.status == "completed";

    let style = Style::default();
    let style = if is_completed {
        style.crossed_out()
    } else {
        style
    };
    // let style = if task.deleted { style.dim() } else { style }; // TODO

    ListItem::new(Line::from(vec![
        Span::raw(if is_completed { "" } else { "" }),
        Span::raw(" "),
        Span::styled(task.title.clone(), style),
    ]))
    // let mut text = Text::from(line);
    // text.extend(
    //     task.children
    //         .iter()
    //         .flat_map(|t| task_to_list_item(t, depth + 1)),
    // );
    // text
}
