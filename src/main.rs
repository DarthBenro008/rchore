mod cli;
mod handlers;
mod models;
mod oauth;
mod printer;
mod secrets;
mod service;
#[macro_use]
extern crate prettytable;

use cli::{CommandLineArgs, Commands::*, GoogleAction::*, TaskAction::*};
use dotenv::dotenv;
use handlers::task_handler::TaskManager;
use handlers::tasklist_handler::TaskListManager;
use service::database_api::TasksDatabase;
use service::google_api::GoogleApiClient;
use service::offline_service::show_stats;
use structopt::StructOpt;

fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let CommandLineArgs { cmd } = CommandLineArgs::from_args();

    let tasks_database = TasksDatabase::new();

    match cmd {
        Tasks { action } => match action {
            List { force } => generate_task_manager(tasks_database).list_tasks(force, false)?,
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
            Stats { shrink } => show_stats(tasks_database, shrink)?,
        },
        TaskList { action } => match action {
            cli::TaskListAction::List => {
                generate_tasklist_manager(tasks_database).list_tasklist()?
            }
            cli::TaskListAction::Delete => {
                generate_tasklist_manager(tasks_database).delete_tasklist()?
            }
            cli::TaskListAction::Add => generate_tasklist_manager(tasks_database).add_tasklist()?,
            cli::TaskListAction::Update => {
                generate_tasklist_manager(tasks_database).update_tasklist()?
            }
        },
        Google { action } => match action {
            Login => {
                oauth::oauth_login(&tasks_database)?;
                generate_task_manager(tasks_database).list_tasks(false, true)?;
            }
            Status => oauth::get_user_info(&tasks_database)?,
            Logout => oauth::logout(&tasks_database)?,
        },
    }
    Ok(())
}

fn generate_task_manager(tasks_database: TasksDatabase) -> TaskManager {
    let google_api_client = GoogleApiClient::new(tasks_database);
    TaskManager {
        client: google_api_client,
    }
}

fn generate_tasklist_manager(tasks_database: TasksDatabase) -> TaskListManager {
    let google_api_client = GoogleApiClient::new(tasks_database);
    TaskListManager {
        client: google_api_client,
    }
}
