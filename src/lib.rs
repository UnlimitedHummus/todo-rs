use std::fmt;
use std::result::Result;
use std::str::FromStr;

pub mod command;

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
