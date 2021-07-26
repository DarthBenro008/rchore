use crate::models::tasks::Tasks;
use crate::service::google_api::GoogleApiClient;
use crate::service::google_tasklist::ApiTaskList;
use crate::service::google_tasks::ApiTasks;
use anyhow;
use console::Term;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use reqwest::header;
use std::env;

pub struct TaskManager {
    pub client: GoogleApiClient,
}

impl TaskManager {
    pub fn list_tasks(&self) -> anyhow::Result<()> {
        let resp = &self.client.fetch_all_tasks(false);
        match resp {
            Ok(list) => {
                let mut order = 1;
                for tasks in &list.items {
                    println!("{}: {}", order, tasks);
                    order += 1;
                }
            }
            Err(err) => println!("Some error occured in fetching tasks! {}", err),
        }
        Ok(())
    }

    pub fn add_task(&self) -> anyhow::Result<()> {
        let title: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Title of the task")
            .with_initial_text("task")
            .allow_empty(false)
            .interact_text()?;
        let notes: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Note for task")
            .with_initial_text("note")
            .allow_empty(true)
            .interact_text()?;
        let items = vec!["No", "Yes"];
        let completed = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Is the task completed?")
            .items(&items)
            .default(0)
            .interact_on_opt(&Term::stderr())?
            .unwrap();
        let status = if completed == 0 {
            String::from("needsAction")
        } else {
            String::from("completed")
        };
        let task = Tasks::new(None, title, notes, status);
        let resp = &self.client.add_task(task);
        match resp {
            Ok(task) => println!("Task {} has been created!", task.title),
            Err(err) => println!("Some error hass occured! {}", err),
        }
        Ok(())
    }

    pub fn show_task(&self, pos: usize) -> anyhow::Result<()> {
        let resp = &self.client.fetch_all_tasks(false);
        match resp {
            Ok(list) => {
                let task = &list.items.get(pos - 1).unwrap().id.as_ref().unwrap();
                let new_resp = &self.client.fetch_task(task.to_string());
                match new_resp {
                    Ok(task) => println!("Task: {}", task),
                    Err(err) => println!("Some error has occured! {}", err),
                }
            }
            Err(err) => println!("Some error occured in fetching tasks! {}", err),
        }
        Ok(())
    }

    pub fn complete_task(&self, pos: usize, is_completed: bool) -> anyhow::Result<()> {
        let resp = &self.client.fetch_all_tasks(false);
        match resp {
            Ok(list) => {
                let mut task = list.items.get(pos - 1).unwrap().clone();
                task.status = if is_completed {
                    String::from("completed")
                } else {
                    String::from("needsAction")
                };
                let new_resp = &self.client.update_task(task);
                match new_resp {
                    Ok(task) => {
                        if is_completed {
                            println!("Task {} marked as completed!", task.title)
                        } else {
                            println!("Task {} marked as incomplete!", task.title)
                        }
                    }
                    Err(err) => println!("Some error occured {}", err),
                }
            }
            Err(err) => println!("Some error occured in fetching tasks! {}", err),
        }
        Ok(())
    }

    pub fn clear_tasks(&self) -> anyhow::Result<()> {
        let resp = &self.client.clear_completed_tasks();
        match resp {
            Ok(()) => println!("Cleared all the tasks!"),
            Err(err) => println!("Some error occured in fetching tasks! {}", err),
        }
        Ok(())
    }

    pub fn delete_task(&self, pos: usize) -> anyhow::Result<()> {
        let resp = &self.client.fetch_all_tasks(false);
        match resp {
            Ok(list) => {
                let task = &list.items.get(pos - 1).unwrap().id.as_ref().unwrap();
                let new_resp = &self.client.delete_task(task.to_string());
                println!("{:#?}", new_resp);
            }
            Err(err) => println!("Some error occured in fetching tasks! {}", err),
        }
        Ok(())
    }
}

pub fn test_fetch() -> anyhow::Result<()> {
    let token = env::var("ID").unwrap();
    let formatted_token = format!("{} {}", "Bearer ", token);
    println!("{}", formatted_token);
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&formatted_token).unwrap(),
    );
    let reqwest_client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()?;
    let mut google_api_client = GoogleApiClient {
        base_url: String::from("https://tasks.googleapis.com/tasks/v1"),
        client: reqwest_client,
        tasklist: None,
    };
    let resp = google_api_client.fetch_tasklist();
    match resp {
        Ok(task_response) => {
            let first_tasklist = task_response.items.get(0);
            match first_tasklist {
                Some(task_list) => {
                    google_api_client.tasklist = Some(String::from(task_list.id.as_ref().unwrap()))
                }
                _ => println!("Some error occured in fetching tasklists"),
            }
        }
        _ => println!("{:#?}", resp),
    }
    let new_resp = google_api_client
        .fetch_task("MDI5MDM4MTYwNzQzNjY1MTk0NTc6MDoyNzY5NTcwMTEzOTYwNzE4".to_string());
    println!("{:#?}", new_resp);
    Ok(())
}
