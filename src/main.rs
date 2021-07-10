mod cli;
use structopt::StructOpt;
fn main() {
    cli::CommandLineArgs::from_args();
}