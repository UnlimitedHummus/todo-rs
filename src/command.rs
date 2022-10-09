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

pub fn remove(file_path: &std::path::Path, item_index: usize, writer: &mut impl std::io::Write) {
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
