use clap::Parser;
use clap::Subcommand;
use std::path::Path;
use todo_rs::command::*;

/// Simple todo lists
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    command: Commands,
}

impl Args {
    pub fn execute_command(&self) {
        self.command.execute();
    }
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Create a new todo list
    Create,
    /// Add an item to the todo list
    Add {
        /// Text of the new item
        #[clap(value_parser)]
        text: String,
    },
    /// List all items on the list
    List,
    /// Mark an item on the list as done
    Check {
        /// Index of the item to check
        #[clap(value_parser)]
        item_index: usize,
    },
    /// Remove an item from the list
    Remove {
        /// Index of the item to remove
        #[clap(value_parser)]
        item_index: usize,
    },
    /// Destroy the todo list
    Destroy,
}

impl Commands {
    pub fn execute(&self) {
        // TODO: add a Result as return value
        match self {
            Commands::Create => match create(Path::new(".")) {
                Ok(_) => println!("created a new .todo file"),
                Err(_) => println!("Warning: \".todo\" already exists. Quitting"),
            },
            Commands::Add { text } => {
                add(Path::new(".todo"), text, &mut std::io::stderr());
            }
            Commands::List => {
                list(Path::new(".todo"), &mut std::io::stdout());
            }
            Commands::Check { item_index } => {
                check(Path::new(".todo"), *item_index, &mut std::io::stdout());
            }
            Commands::Remove { item_index } => {
                remove(Path::new(".todo"), *item_index, &mut std::io::stdout());
            }
            Commands::Destroy => {
                destroy(Path::new(".todo"), &mut std::io::stdout());
            }
        }
    }
}
