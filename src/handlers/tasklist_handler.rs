use crate::models::tasklist::TaskList;
use crate::printer::{print_error, print_ok, print_warning};
use crate::service::google_api::GoogleApiClient;
use crate::service::google_tasklist::ApiTaskList;
use anyhow::anyhow;
use console::Term;
use dialoguer::{theme::ColorfulTheme, Input, Select};

pub struct TaskListManager {
    pub client: GoogleApiClient,
}

impl TaskListManager {
    pub fn list_tasklist(&self) -> anyhow::Result<()> {
        let task = self.select_tasklist()?;
        self.client
            .localdb
            .insert_default_tasklist(task.id.unwrap(), task.title.clone())?;
        print_ok(format!(
            "The tasklist {} has been set as default!",
            &task.title
        ));
        Ok(())
    }

    pub fn add_tasklist(&self) -> anyhow::Result<()> {
        let title: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Title of the task-list")
            .with_initial_text("My tasklist")
            .allow_empty(false)
            .interact_text()?;
        let resp = &self.client.create_tasklist(title);
        match resp {
            Ok(value) => {
                print_ok(format!("Task List {} created!", &value.title));
                let items = vec!["No", "Yes"];
                let completed = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Do you want to make this the default tasklist?")
                    .items(&items)
                    .default(0)
                    .interact_on_opt(&Term::stderr())?
                    .unwrap();
                if completed == 0 {
                    print_ok("Ok, did not set it as default".to_string());
                } else {
                    self.client.localdb.insert_default_tasklist(
                        String::from(&value.id.as_ref().unwrap().to_string()),
                        String::from(&value.title),
                    )?;
                    print_ok(format!("Default task-list set to {}", value.title))
                };
            }
            Err(err) => print_error("creating tasklist", err),
        }
        Ok(())
    }

    pub fn update_tasklist(&self) -> anyhow::Result<()> {
        let mut tasklist = self.select_tasklist()?;
        let title: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Title of the task-list")
            .with_initial_text("My tasklist")
            .allow_empty(false)
            .interact_text()?;
        tasklist.title = title;
        let resp = &self.client.update_tasklist(
            String::from(&tasklist.id.unwrap()),
            String::from(&tasklist.title),
        );
        match resp {
            Ok(value) => print_ok(format!(
                "Task List {} updated to {}",
                tasklist.title, value.title
            )),
            Err(err) => print_error("updating tasklist", err),
        }
        Ok(())
    }

    pub fn delete_tasklist(&self) -> anyhow::Result<()> {
        let tasklist = self.select_tasklist()?;
        let selection: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Are you sure you want to delete this tasklist?")
            .with_initial_text("n")
            .default("n".into())
            .allow_empty(false)
            .interact_text()?;
        if selection == "n" {
            print_warning("Aborting delete of tasklist".to_string())
        } else {
            let resp = &self.client.delete_tasklist(tasklist.id.unwrap());
            match resp {
                Ok(_) => print_ok(format!(
                    "Task-List {} deleted successfully!",
                    tasklist.title
                )),
                Err(err) => print_error("deleting tasklist", err),
            }
        }
        Ok(())
    }

    fn get_tasklist(&self) -> anyhow::Result<Vec<TaskList>> {
        let resp = &self.client.fetch_tasklist();
        match resp {
            Ok(data) => Ok(data.items.clone()),
            Err(_err) => Err(anyhow!("Cannot fetch tasklists!")),
        }
    }

    fn select_tasklist(&self) -> anyhow::Result<TaskList> {
        let tasklists = &self.get_tasklist()?;
        let mut list = Vec::new();
        for tasklist in tasklists {
            list.push(&tasklist.title);
        }
        let completed = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select a default tasklist")
            .items(&list)
            .default(0)
            .interact_on_opt(&Term::stderr())?
            .unwrap();
        Ok(tasklists.get(completed).unwrap().clone())
    }
}
