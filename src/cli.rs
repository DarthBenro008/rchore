use structopt::StructOpt;

#[derive(Debug, StructOpt, PartialEq)]
pub enum LocalAction {
    List {
        #[structopt(short, long)]
        force: bool,
    },
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
