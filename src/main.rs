use clap::Parser;
use clap::Subcommand;
use todo_rs::create;

/// Simple todo lists
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// create a new list
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// creates a new todo list
    Create,
}

fn main() {
    let args = Args::parse();

    println!("created a new .todo.toml file")
}
