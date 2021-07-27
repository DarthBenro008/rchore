use structopt::StructOpt;

#[derive(Debug, StructOpt, PartialEq)]
pub enum TaskAction {
    /// Show all tasks in a task-list.
    List {
        #[structopt(short, long)]
        force: bool,
    },
    /// Mark the given task completed.
    Done {
        #[structopt()]
        position: usize,
    },
    /// Unmark the given task completed.
    Undo {
        #[structopt()]
        position: usize,
    },
    /// Deletes the given task from the task-list.
    Delete {
        #[structopt()]
        position: usize,
    },
    /// Shows details about the given task completed.
    Show {
        #[structopt()]
        position: usize,
    },
    /// Create a new task in the task-list.
    Add,
    /// Clear all completed tasks in a task-list.
    Clear,
}

#[derive(Debug, StructOpt, PartialEq)]
pub enum TaskListAction {
    /// Lists all task-lists and asks for default task-list selection.
    List,
    /// Deletes a task-list.
    Delete,
    /// Creates a task-list.
    Add,
    /// Updates a task-list.
    Update,
}

#[derive(Debug, StructOpt, PartialEq)]
pub enum GoogleAction {
    /// Authenticate yourself via Google.
    Login,
}

#[derive(Debug, StructOpt, PartialEq)]
/// An Unofficial Google Tasks CLI written purely in Rust
///         
///        * Manage your Google Tasks right from your terminal!
///        * Select and manage from various task-lists
///        * Run `rchore google login` to get started
///
/// Developed by Hemanth Krishna (https://github.com/DarthBenro008)
#[structopt(
    name = "rChore",
    about = "An Unofficial Google Tasks CLI written in Rust",
    verbatim_doc_comment
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub cmd: Commands,
}

#[derive(Debug, StructOpt, PartialEq)]
pub enum Commands {
    /// Helps to Create, Read, Update and Delete tasks.
    Tasks {
        #[structopt(subcommand)]
        action: TaskAction,
    },
    /// Helps to authenticate into Google and check status.
    Google {
        #[structopt(subcommand, about = "I am a program and I work, just pass `-h`")]
        action: GoogleAction,
    },
    /// Helps to select, Update and Delete task-lists.
    TaskList {
        #[structopt(subcommand)]
        action: TaskListAction,
    },
}
