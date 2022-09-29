use std::fmt;
use std::fs::File;
use std::result::Result;
use std::str::FromStr;

#[derive(Debug)]
pub enum Error {
    FileExists,
}

#[derive(Debug, PartialEq)]
struct Task {
    text: String,
    status: Status,
}

#[derive(PartialEq, Debug)]
enum Status {
    Unfinished,
    Finished,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let representation = match self {
            Status::Unfinished => "[ ]",
            Status::Finished => "[x]",
        };
        write!(f, "{}", representation)
    }
}

#[derive(Debug)]
struct ParseError;

impl FromStr for Task {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, <Self as FromStr>::Err> {
        let status = match &input[0..3] {
            "[ ]" => Status::Unfinished,
            "[x]" => Status::Finished,
            _ => return Err(ParseError),
        };
        let text = &input[4..input.len()];

        Ok(Task {
            text: text.to_string(),
            status,
        })
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.status.to_string(), self.text)
    }
}

pub fn create(path: &std::path::Path) -> Result<(), Error> {
    let file_path = path.join(".todo.toml");
    if file_path.exists() {
        Err(Error::FileExists)
    } else {
        File::create(path.join(".todo.toml")).expect("File could not be created");
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{create, Status, Task};
    use assert_fs::fixture::TempDir;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;

    #[test]
    fn test_create_function_creates_a_new_file() {
        let temp_dir = TempDir::new().unwrap();

        create(temp_dir.path()).unwrap();

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

    #[test]
    fn test_parse_unfinished_task() {
        let task = "[ ] Make things work"
            .parse::<Task>()
            .expect("parsing failed");

        assert_eq!(
            Task {
                text: "Make things work".to_string(),
                status: Status::Unfinished
            },
            task
        );
    }

    #[test]
    fn test_parse_finished_task() {
        let task = "[x] Nothing special"
            .parse::<Task>()
            .expect("parsing failed");

        assert_eq!(
            Task {
                text: "Nothing special".to_string(),
                status: Status::Finished
            },
            task
        );
    }

    #[test]
    fn test_finished_task_to_string() {
        let task = Task {
            text: "Get coffee".to_string(),
            status: Status::Finished,
        };

        assert_eq!("[x] Get coffee", task.to_string());
    }

    #[test]
    fn test_unfinished_task_to_string() {
        let task = Task {
            text: "Get coffee".to_string(),
            status: Status::Unfinished,
        };

        assert_eq!("[ ] Get coffee", task.to_string());
    }
}
