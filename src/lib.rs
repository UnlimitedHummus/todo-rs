use std::fmt;
use std::result::Result;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Error {
    FileExists,
    IndexOutOfBounds,
}

#[derive(Debug, PartialEq, Clone)]
struct Task {
    text: String,
    status: Status,
}

#[derive(PartialEq, Debug, Clone)]
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

impl Task {
    fn is_finished(&self) -> bool {
        match self.status {
            Status::Finished => true,
            Status::Unfinished => false,
        }
    }

    fn check(&mut self) -> Task {
        self.status = Status::Finished;
        self.clone()
    }
}

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
        write!(f, "{} {}", self.status, self.text)
    }
}

#[derive(PartialEq, Debug)]
struct TaskList {
    tasks: Vec<Task>,
}

impl FromStr for TaskList {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, <Self as FromStr>::Err> {
        let tasks = input
            .lines()
            .map(|line| line.parse::<Task>().unwrap())
            .collect();
        Ok(TaskList { tasks })
    }
}

impl TaskList {
    fn new() -> Self {
        TaskList { tasks: vec![] }
    }

    fn add(&mut self, new_task: Task) -> &mut Self {
        self.tasks.push(new_task);
        self
    }

    // TODO: replace with a filter function to filter for status
    fn finished_tasks(&self) -> Vec<Task> {
        self.tasks
            .iter()
            .filter(|x| x.is_finished())
            .map(|x| x.to_owned())
            .collect()
    }

    fn unfinished_tasks(&self) -> Vec<Task> {
        self.tasks
            .iter()
            .filter(|x| !x.is_finished())
            .map(|x| x.to_owned())
            .collect()
    }

    fn to_string_unordered(&self) -> String {
        self.tasks
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("\n")
            + "\n"
    }

    fn check(&mut self, task_index: usize) -> Result<Task, Error> {
        if task_index == 0 || task_index - 1 > self.tasks.len() {
            Err(Error::IndexOutOfBounds)
        } else {
            let mut tasks = self.unfinished_tasks();
            tasks.append(&mut self.finished_tasks());
            let task_to_check = tasks.get(task_index - 1).unwrap();
            let task = self.tasks.iter_mut().find_map(|task| {
                if task == task_to_check {
                    Some(task.check())
                } else {
                    None
                }
            });
            Ok(task.unwrap())
        }
    }

    fn remove(&mut self, task_index: usize) -> Result<Task, Error> {
        if task_index == 0 || task_index - 1 > self.tasks.len() {
            Err(Error::IndexOutOfBounds)
        } else {
            let mut tasks = self.unfinished_tasks();
            tasks.append(&mut self.finished_tasks());
            let task_to_remove = tasks.get(task_index - 1).unwrap();
            self.tasks = self
                .tasks
                .iter()
                .filter(|task| &task_to_remove != task)
                .map(|task| task.to_owned())
                .collect();

            Ok(task_to_remove.to_owned())
        }
    }
}

impl fmt::Display for TaskList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut counter = 1;
        for unfinished_task in self.unfinished_tasks().iter() {
            writeln!(f, "{} {}", counter, unfinished_task)?;
            counter += 1;
        }
        writeln!(f)?;
        for finished_task in self.finished_tasks().iter() {
            writeln!(f, "{} {}", counter, finished_task)?;
            counter += 1;
        }
        Ok(()) // TODO: can I remove the loops in favor of iterators
    }
}

pub mod command {
    use crate::Error;
    use crate::TaskList;
    use std::fs;
    use std::fs::File;
    use std::fs::OpenOptions;
    use std::io::Write;

    pub fn create(path: &std::path::Path) -> Result<(), Error> {
        let file_path = path.join(".todo");
        if file_path.exists() {
            Err(Error::FileExists)
        } else {
            File::create(path.join(".todo")).expect("File could not be created");
            Ok(())
        }
    }

    pub fn list(file_path: &std::path::Path, writer: &mut impl std::io::Write) {
        let file_content = fs::read_to_string(file_path).expect("Couldn't read file contents");
        let task_list = file_content.parse::<TaskList>().unwrap();
        write!(writer, "{}", task_list)
            .unwrap_or_else(|_| panic!("Error parsing file {}", file_path.display()));
    }

    pub fn add(file_path: &std::path::Path, text: &str, error_writer: &mut impl std::io::Write) {
        match OpenOptions::new().append(true).open(file_path) {
            Ok(mut file) => {
                file.write_all(b"[ ] ").unwrap();
                file.write_all(text.as_bytes()).unwrap();
                file.write_all(b"\n").unwrap();
                println!("Added: {}", text);
            }
            Err(_) => {
                writeln!(error_writer, "Error: There is no list in this directory").unwrap();
                std::process::exit(1);
            }
        }
    }

    pub fn check(file_path: &std::path::Path, item_index: usize, writer: &mut impl std::io::Write) {
        let file_content = fs::read_to_string(file_path).expect("Couldn't read file contents"); // TODO: make these two lines their own function
        let mut task_list = file_content.parse::<TaskList>().unwrap();
        let checked_task = task_list.check(item_index).unwrap();
        fs::write(file_path, task_list.to_string_unordered()).unwrap();
        writeln!(writer, "{}", checked_task).unwrap();
        // TODO: maybe use mark or toggle instead of check
    }

    pub fn remove(
        file_path: &std::path::Path,
        item_index: usize,
        writer: &mut impl std::io::Write,
    ) {
        let file_content = fs::read_to_string(file_path).expect("Couldn't read file contents"); // TODO: make these two lines their own function
        let mut task_list = file_content.parse::<TaskList>().unwrap();
        let removed_task = task_list.remove(item_index).unwrap();
        fs::write(file_path, task_list.to_string_unordered()).unwrap();
        writeln!(writer, "Removed: {}", removed_task).unwrap();
        // TODO: make a warning for trying to remove an Item with the wrong index
    }

    pub fn destroy(file_path: &std::path::Path, writer: &mut impl std::io::Write) {
        std::fs::remove_file(file_path).unwrap();
        writeln!(writer, "Deleted: .todo").unwrap();
    }

    #[cfg(test)]
    mod test {
        use super::*;
        use assert_fs::fixture::TempDir;
        use assert_fs::fixture::{FileTouch, NamedTempFile};
        use std::fs::{self, read_to_string, File};
        use std::io::Write;
        use std::path::Path;

        #[test]
        fn test_add_appends_text_to_file() {
            let temp_file = NamedTempFile::new(".todo").unwrap();
            let text = "New todo entry".to_string();
            temp_file.touch().unwrap();
            {
                let mut f = File::options().write(true).open(temp_file.path()).unwrap();
                f.write(b"[x] Old todo entry\n").unwrap();
            }

            add(temp_file.path(), &text, &mut std::io::stderr());

            assert_eq!(
                read_to_string(temp_file.path()).unwrap(),
                "[x] Old todo entry\n[ ] New todo entry\n".to_string()
            );
            temp_file.close().unwrap();
        }

        #[test]
        fn test_create_function_creates_a_new_file() {
            let temp_dir = TempDir::new().unwrap();

            create(temp_dir.path()).unwrap();

            assert!(temp_dir.path().join(Path::new(".todo")).exists());

            temp_dir.close().unwrap();
        }

        #[test]
        fn test_create_function_doesnt_overwrite_existing_file() {
            let temp_dir = TempDir::new().unwrap();
            let file_path = temp_dir.path().join(".todo");
            {
                let mut file = File::create(file_path.clone()).unwrap();
                file.write(b"foo").unwrap();
            }

            let result = create(temp_dir.path());

            assert!(result.is_err());

            // file contents should not be altered
            let file_content = fs::read_to_string(file_path).unwrap();
            assert_eq!(file_content, "foo");
        }

        #[test]
        fn test_add_writes_to_file() {
            let temp_file = NamedTempFile::new(".todo").unwrap();
            let text = "New todo entry".to_string();
            temp_file.touch().unwrap();

            add(temp_file.path(), &text, &mut std::io::stderr());

            assert_eq!(
                read_to_string(temp_file.path()).unwrap(),
                "[ ] New todo entry\n".to_string()
            );
            temp_file.close().unwrap();
        }

        #[test]
        fn test_list_subcommand() {
            let temp_dir = TempDir::new().unwrap();
            let file_path = temp_dir.path().join(".todo");
            std::fs::write(file_path.clone(), "[x] Already done\n[ ] Not done yet\n").unwrap();
            let mut writer = Vec::<u8>::new();

            list(&file_path, &mut writer);

            assert_eq!(
                "1 [ ] Not done yet\n\n2 [x] Already done\n",
                String::from_utf8(writer).unwrap()
            );
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;
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

    #[test]
    fn test_parse_and_display_a_list_of_tasks() {
        let mut tasks = TaskList::new();
        let task1 = Task {
            text: "Do this task first".to_string(),
            status: Status::Finished,
        };
        let task2 = Task {
            text: "Then do this task".to_string(),
            status: Status::Unfinished,
        };
        let task3 = Task {
            text: "Finally do this task".to_string(),
            status: Status::Unfinished,
        };
        tasks.add(task1);
        tasks.add(task2);
        tasks.add(task3);

        assert_eq!(
            r"1 [ ] Then do this task
2 [ ] Finally do this task

3 [x] Do this task first
",
            tasks.to_string()
        );
    }

    #[test]
    fn test_task_list_finished() {
        let mut tasks = TaskList::new();
        let task1 = Task {
            text: "Do this task first".to_string(),
            status: Status::Finished,
        };
        let task2 = Task {
            text: "Then do this task".to_string(),
            status: Status::Unfinished,
        };
        tasks.add(task1.clone());
        tasks.add(task2.clone());

        assert_eq!(tasks.finished_tasks(), vec![task1]);
    }

    #[test]
    fn test_is_finished() {
        let task1 = Task {
            text: "Do this task first".to_string(),
            status: Status::Finished,
        };
        let task2 = Task {
            text: "Then do this task".to_string(),
            status: Status::Unfinished,
        };

        assert!(task1.is_finished());
        assert!(!task2.is_finished());
    }

    #[test]
    fn test_task_list_unfinished() {
        let mut tasks = TaskList::new();
        let task1 = Task {
            text: "Do this task first".to_string(),
            status: Status::Finished,
        };
        let task2 = Task {
            text: "Then do this task".to_string(),
            status: Status::Unfinished,
        };
        tasks.add(task1.clone());
        tasks.add(task2.clone());

        assert_eq!(tasks.unfinished_tasks(), vec![task2]);
    }

    #[test]
    fn test_parse_task_list() {
        let tasks = "[ ] Task 1\n[x] Task 2".parse::<TaskList>().unwrap();
        let task1 = "[ ] Task 1".parse().unwrap();
        let task2 = "[x] Task 2".parse().unwrap();
        let mut expected_tasks = TaskList::new();
        expected_tasks.add(task1);
        expected_tasks.add(task2);

        assert_eq!(expected_tasks, tasks);
    }

    #[test]
    fn test_to_string_unordered() {
        let tasks = "[ ] Task 1\n[x] Task 2".parse::<TaskList>().unwrap();
        assert_eq!(tasks.to_string_unordered(), "[ ] Task 1\n[x] Task 2\n");
    }

    #[test]
    fn test_check_marks_task_and_returns_task() {
        let mut tasks = "[ ] Task 1\n[x] Task 2".parse::<TaskList>().unwrap();

        let task = tasks.check(1).unwrap();
        assert_eq!("[x] Task 1\n[x] Task 2\n", tasks.to_string_unordered());
        assert_eq!(
            Task {
                text: "Task 1".to_string(),
                status: Status::Finished
            },
            task
        );
    }
    // TODO: make unmarking also possible

    #[test]
    fn test_check_indexes_correctly() {
        let mut tasks = "[x] Task 2\n[ ] Task 1".parse::<TaskList>().unwrap();

        let checked_task = tasks.check(1).unwrap();
        assert_eq!("[x] Task 2\n[x] Task 1\n", tasks.to_string_unordered());
        assert_eq!(
            Task {
                text: "Task 1".to_string(),
                status: Status::Finished
            },
            checked_task
        );
    }

    #[test]
    fn test_check_returns_out_of_bounds_error() {
        let mut tasks = "[ ] Task 1\n[x] Task 2".parse::<TaskList>().unwrap();
        let result = tasks.check(5);
        assert_eq!(Err(Error::IndexOutOfBounds), result);
    }

    #[test]
    fn test_task_list_remove_item() {
        let mut tasks = r"[x] Read a book from a series of books
[x] Buy all of the books from the series
[ ] Read all the books from the book series"
            .parse::<TaskList>()
            .unwrap();
        let task2 = tasks.remove(3).unwrap();
        assert_eq!(
            Task {
                text: "Buy all of the books from the series".to_string(),
                status: Status::Finished
            },
            task2
        );
        assert_eq!(
            vec![
                Task {
                    text: "Read a book from a series of books".to_string(),
                    status: Status::Finished
                },
                Task {
                    text: "Read all the books from the book series".to_string(),
                    status: Status::Unfinished
                }
            ],
            tasks.tasks
        );

        let task3 = tasks.remove(1).unwrap();
        assert_eq!(
            Task {
                text: "Read all the books from the book series".to_string(),
                status: Status::Unfinished
            },
            task3
        );
        assert_eq!(
            vec![Task {
                text: "Read a book from a series of books".to_string(),
                status: Status::Finished
            }],
            tasks.tasks
        );

        assert_eq!(Err(Error::IndexOutOfBounds), tasks.remove(5));
    }

    #[test]
    fn test_remove_takes_index_as_displayed_not_as_in_file() {
        let mut tasks = "[x] Task2\n[ ] Task1".parse::<TaskList>().unwrap();

        let task1 = tasks.remove(1).unwrap();

        assert_eq!(task1, "[ ] Task1".parse::<Task>().unwrap());
        assert_eq!("[x] Task2".parse::<TaskList>().unwrap(), tasks);
    }
}
