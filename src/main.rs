use clap::Parser;
use clap::Subcommand;

/// Simple todo lists
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// create a new list
    #[clap(subcommand)]
    command: Commands
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// creates a new todo list
    Create
}

fn main() {
    let args = Args::parse();

    println!("created a new .todo.toml file")
}

#[cfg(test)]
mod test {
    #[test]
    fn test_create_function_creates_a_new_file() {
        todo!("Finish this test")
    }
}
