mod cli;
mod tasks;

use structopt::StructOpt;
use cli::{Action::*, CommandLineArgs};
use tasks::Task;
use std::path::PathBuf;
use anyhow::anyhow;

fn default_local_journal() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rchore.json");
        path
    })
}

fn main() -> anyhow::Result<()>{
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    let journal_file = journal_file.or_else(default_local_journal).ok_or(anyhow!("Failed to find rchore journal"))?;

    match action {
        Add {text} => tasks::add_task(journal_file, Task::new(text)),
        Done {position} => tasks::complete_task(journal_file, position),
        List => tasks::list_tasks(journal_file)
    }?;
    Ok(())
}