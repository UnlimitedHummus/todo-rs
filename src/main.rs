use clap::Parser;
use clap::Subcommand;
use std::fs::File;
use std::result::Result;


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

#[derive(Debug)]
enum Error {
    FileExists
}


fn main() {
    let args = Args::parse();

    println!("created a new .todo.toml file")
}

fn create(path: &std::path::Path) -> Result<(), Error> {
    let file_path = path.join(".todo.toml");
    if file_path.exists() {
        Err(Error::FileExists)
    }
    else{
        File::create(path.join(".todo.toml")).expect("File could not be created");
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::create;
    use std::path::Path;
    use std::io::{Read, Write};
    use std::fs::File;
    use assert_fs::fixture::TempDir;

    #[test]
    fn test_create_function_creates_a_new_file() {
        let temp_dir = TempDir::new().unwrap();

        create(temp_dir.path());

        assert!(temp_dir.path().join(Path::new(".todo.toml")).exists());

        temp_dir.close().unwrap();
    }

    #[test]
    fn test_create_function_doesnt_overwrite_existing_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join(".todo.toml");
        {
            let mut file = File::create(file_path.clone()).unwrap();
            file.write(b"foo").unwrap();
        }

        let result = create(temp_dir.path());

        assert!(result.is_err());

        // file contents should not be altered
        let file_content = std::fs::read_to_string(file_path).unwrap();
        assert_eq!(file_content, "foo");
    }
        
}
