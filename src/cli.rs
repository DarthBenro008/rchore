use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt, PartialEq)]
pub enum LocalAction {
    // Add {
    //     #[structopt()]
    //     text: String,
    // },

    // Done {
    //     #[structopt()]
    //     position: usize,
    // },
    // List,
    // Fetch,
    List,
    Done {
        #[structopt()]
        position: usize,
    },
    Undo {
        #[structopt()]
        position: usize,
    },
    Delete {
        #[structopt()]
        position: usize,
    },
    Show {
        #[structopt()]
        position: usize,
    },
    Add,
    Clear,
}

#[derive(Debug, StructOpt, PartialEq)]
pub enum GoogleAction {
    Login,
}

#[derive(Debug, StructOpt, PartialEq)]
#[structopt(name = "R Chore", about = "A command line to-do app written in Rust")]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub cmd: Commands,

    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>,
}

#[derive(Debug, StructOpt, PartialEq)]
pub enum Commands {
    Tasks {
        #[structopt(subcommand)]
        action: LocalAction,
    },
    Google {
        #[structopt(subcommand)]
        action: GoogleAction,
    },
}
