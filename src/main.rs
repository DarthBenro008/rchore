mod cli;
mod controller;
mod models;
mod oauth;
mod service;
mod tasks;

use anyhow::anyhow;
use cli::{CommandLineArgs, Commands::*, GoogleAction::*, LocalAction::*};
use controller::TaskManager;
use dotenv::dotenv;
use service::database_api::TasksDatabase;
use service::google_api::GoogleApiClient;
use std::path::PathBuf;
use structopt::StructOpt;

fn default_local_journal() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rchore.json");
        path
    })
}

fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let CommandLineArgs { cmd, journal_file } = CommandLineArgs::from_args();

    let _journal_file = journal_file
        .or_else(default_local_journal)
        .ok_or(anyhow!("Failed to find rchore journal"))?;

    let tasks_database = TasksDatabase::new();

    match cmd {
        Tasks { action } => match action {
            List => generate_task_manager(tasks_database).list_tasks()?,
            Done { position } => {
                generate_task_manager(tasks_database).complete_task(position, true)?
            }
            Delete { position } => generate_task_manager(tasks_database).delete_task(position)?,
            Show { position } => generate_task_manager(tasks_database).show_task(position)?,
            Add => generate_task_manager(tasks_database).add_task()?,
            Clear => generate_task_manager(tasks_database).clear_tasks()?,
            Undo { position } => {
                generate_task_manager(tasks_database).complete_task(position, false)?
            }
        },
        Google { action } => match action {
            Login => oauth::oauth_login(&tasks_database)?,
        },
    }
    Ok(())
}

fn generate_task_manager(tasks_database: TasksDatabase) -> TaskManager {
    let google_api_client = GoogleApiClient::new(tasks_database);
    return TaskManager {
        client: google_api_client,
    };
}
