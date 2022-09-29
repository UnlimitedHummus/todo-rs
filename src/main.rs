use clap::Parser;
use clap::Subcommand;
use std::path::Path;
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
    /// Add an item to the todo list
    Add{
        /// text of todo list item
        #[clap(value_parser)]
        text: String,
    },
    /// List all items on the list
    List,
}

fn main() {
    let args = Args::parse();
    match args.command {
        Commands::Create => {
            match create(Path::new(".")) {
            Ok(_) => println!("created a new .todo.toml file"),
            Err(_) => println!("Warning: \".todo.toml\" already exists. Quitting") 
            }

        },
        Commands::Add{ text } => {
            println!("Added: {}", text);
        },
        Commands::List => {
            println!("1 [ ] Refactor code");
        }
    }
}
