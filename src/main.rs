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
use service::google_api::GoogleApiClient;
use std::path::PathBuf;
use structopt::StructOpt;
use tasks::Task;

fn default_local_journal() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rchore.json");
        path
    })
}

fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let CommandLineArgs { cmd, journal_file } = CommandLineArgs::from_args();

    let journal_file = journal_file
        .or_else(default_local_journal)
        .ok_or(anyhow!("Failed to find rchore journal"))?;

    let google_api_client = GoogleApiClient::new();
    let task_manager = TaskManager {
        client: google_api_client,
    };

    match cmd {
        Tasks { action } => match action {
            // Add { text } => tasks::add_task(journal_file, Task::new(text))?,
            // Done { position } => tasks::complete_task(journal_file, position)?,
            // List => tasks::list_tasks(journal_file)?,
            // Fetch => controller::test_fetch()?,
            List => task_manager.list_tasks()?,
            Done { position } => task_manager.complete_task(position, true)?,
            Delete { position } => task_manager.delete_task(position)?,
            Show { position } => task_manager.show_task(position)?,
            Add => task_manager.add_task()?,
            Clear => task_manager.clear_tasks()?,
            Undo { position } => task_manager.complete_task(position, false)?,
        },
        Google { action } => match action {
            Login => oauth::oauth_login()?,
        },
    }
    Ok(())
}
