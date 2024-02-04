use crate::models::tasks::Tasks;
use crate::printer::{print_error, print_success, print_task_table, print_warning};
use crate::service::google_api::GoogleApiClient;
use crate::service::google_tasks::ApiTasks;
use anyhow;
use console::Term;
use dialoguer::{theme::ColorfulTheme, Input, Select};

pub struct TaskManager {
    pub client: GoogleApiClient,
}

impl TaskManager {
    pub fn list_tasks(&mut self, show_hidden: bool, is_silent: bool) -> anyhow::Result<()> {
        let resp = self.client.fetch_all_tasks(show_hidden);
        match resp {
            Ok(list) => {
                self.client.localdb.insert_tasks(list.items.clone())?;
                if !is_silent {
                    print_task_table(&list.items);
                }
            }
            Err(err) => print_error("fetching tasks", &err),
        }
        Ok(())
    }

    pub fn add_task(
        &self,
        title: Option<String>,
        notes: Option<String>,
        completed: bool,
    ) -> anyhow::Result<()> {
        let task = match title {
            Some(t) => self.create_task_without_prompts(t, notes, completed),
            None => self.create_task_with_prompts(notes, completed)?,
        };
        let resp = &self.client.add_task(task);
        match resp {
            Ok(task) => {
                let _ = self.client.localdb.add_task(task.clone());
                print_success(format!("Task {} has been created!", task.title))
            }
            Err(err) => print_error("creating task", err),
        }
        Ok(())
    }

    fn create_task_without_prompts(
        &self,
        title: String,
        notes: Option<String>,
        completed: bool,
    ) -> Tasks {
        let status = if completed {
            String::from("completed")
        } else {
            String::from("needsAction")
        };
        Tasks::new(
            None,
            title,
            notes.unwrap_or_else(|| String::from("")),
            status,
        )
    }

    fn create_task_with_prompts(&self, notes: Option<String>, done: bool) -> anyhow::Result<Tasks> {
        let title: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Title of the task")
            .with_initial_text("task")
            .allow_empty(false)
            .interact_text()?;

        let notes: String = notes.map(Result::Ok).unwrap_or_else(|| {
            Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Note for task")
                .with_initial_text("note")
                .allow_empty(true)
                .interact_text()
        })?;

        let items = vec!["No", "Yes"];
        let completed = if done {
            1_usize
        } else {
            Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Is the task completed?")
                .items(&items)
                .default(0)
                .interact_on_opt(&Term::stderr())?
                .unwrap()
        };

        let status = if done || completed == 1 {
            String::from("completed")
        } else {
            String::from("needsAction")
        };

        Ok(Tasks::new(None, title, notes, status))
    }

    pub fn show_task(&self, pos: usize) -> anyhow::Result<()> {
        let resp = &self.client.localdb.get_data()?;
        let task = resp.get(pos - 1).unwrap().id.as_ref().unwrap();
        let new_resp = &self.client.fetch_task(task.to_string());
        match new_resp {
            Ok(task) => print_success(format!("Task: {}", task)),
            Err(err) => print_error("enquiring task", err),
        }
        Ok(())
    }

    pub fn complete_task(&self, pos: usize, is_completed: bool) -> anyhow::Result<()> {
        let resp = &self.client.localdb.get_data()?;
        let mut task = resp.get(pos - 1).unwrap().clone();
        task.status = if is_completed {
            String::from("completed")
        } else {
            String::from("needsAction")
        };
        let new_resp = &self.client.update_task(task);
        match new_resp {
            Ok(task) => {
                let _ = self.client.localdb.update_task(task.clone());
                if is_completed {
                    print_success(format!("Task {} marked as completed!", task.title))
                } else {
                    print_warning(format!("Task {} marked as incomplete!", task.title))
                }
            }
            Err(err) => print_error("marking task complete", err),
        }
        Ok(())
    }

    pub fn clear_tasks(&self) -> anyhow::Result<()> {
        let resp = &self.client.clear_completed_tasks();
        match resp {
            Ok(()) => {
                let _ = self.client.localdb.clear_completed_tasks();
                print_success("Cleared all the tasks!".to_string())
            }
            Err(err) => print_error("clearing completed tasks", err),
        }
        Ok(())
    }

    pub fn delete_task(&self, pos: usize) -> anyhow::Result<()> {
        let resp = &self.client.localdb.get_data()?;
        let task = resp.get(pos - 1).unwrap();
        let new_resp = &self
            .client
            .delete_task(task.id.as_ref().unwrap().to_string());
        match new_resp {
            Ok(_res) => {
                let _ = self
                    .client
                    .localdb
                    .delete_task(task.id.as_ref().unwrap().to_string());
                print_success(format!("Task {} has been deleted!", &task.title))
            }
            Err(err) => print_error("deleting the task", err),
        }
        Ok(())
    }
}
