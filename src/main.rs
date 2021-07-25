mod api;
mod cli;
mod controller;
mod models;
mod oauth;
mod tasks;

use anyhow::anyhow;
use cli::{CommandLineArgs, Commands::*, GoogleAction::*, LocalAction::*};
use dotenv::dotenv;
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

    match cmd {
        Tasks { action } => match action {
            Add { text } => tasks::add_task(journal_file, Task::new(text))?,
            Done { position } => tasks::complete_task(journal_file, position)?,
            List => tasks::list_tasks(journal_file)?,
            Fetch => controller::test_fetch()?,
        },
        Google { action } => match action {
            Login => oauth::oauth_login()?,
        },
    }
    Ok(())
}
