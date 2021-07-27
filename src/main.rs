mod cli;
mod controller;
mod models;
mod oauth;
mod service;
mod tasks;

use cli::{CommandLineArgs, Commands::*, GoogleAction::*, LocalAction::*};
use controller::TaskManager;
use dotenv::dotenv;
use service::database_api::TasksDatabase;
use service::google_api::GoogleApiClient;
use structopt::StructOpt;

fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let CommandLineArgs { cmd } = CommandLineArgs::from_args();

    let tasks_database = TasksDatabase::new();

    match cmd {
        Tasks { action } => match action {
            List { force } => generate_task_manager(tasks_database).list_tasks(force)?,
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
