use structopt::StructOpt;

#[derive(Debug, StructOpt, PartialEq)]
pub enum TaskAction {
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
pub enum TaskListAction {
    List,
    Delete,
    Add,
    Update,
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
        action: TaskAction,
    },
    Google {
        #[structopt(subcommand)]
        action: GoogleAction,
    },
    TaskList {
        #[structopt(subcommand)]
        action: TaskListAction,
    },
}
