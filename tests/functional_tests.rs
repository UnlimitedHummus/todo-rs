// use assert_cmd::prelude::*;
use assert_cmd::Command;
use predicates::prelude::*;
use std::path::Path;

#[test]
fn test_program_fails_when_no_arguments_are_passed() {
    // Mark wants to create a new todo list in the current folder
    // He doesn't know how to use the program yet and is displayed a help
    // menu, because he just typed `todo`
    let mut cmd = Command::cargo_bin("todo-rs").unwrap();
    cmd.assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains("help"));
}

#[test]
fn test_managing_list_items() -> Result<(), Box<dyn std::error::Error>> {
    // Mark wants to create a new todo list in the current folder
    // He runs `todo create` to make a new todo list in the current folder
    // This creates a .todo.toml file in the current folder
    let mut cmd = Command::cargo_bin("todo-rs").unwrap();
    cmd.arg("create");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("created a new .todo.toml file"));
    assert!(Path::new(".todo.toml").exists());

    // Mark uses the todo program to manage tasks for his project.
    // By running todo add "Refactor code" he adds an item to the todo list
    let mut cmd = Command::cargo_bin("todo-rs").unwrap();
    cmd.arg("add").arg("Refactor code");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Added: Refactor code"));

    // Immediately, he decides to view the current state of the todo list
    // by running `todo list`. Which shows him the item he added

    // assert ... "[ ] Refactor code";
    // assert ... 1 in output;

    // He wants to also add a second item to his todo list "Drink a coffe
    // with Greg"

    // He has another look at the todo list, which now shows him both items
    // They are numbered in the order they were added, which makes sense
    // to Mark

    // Mark completes his refacoring and checks of the item on the todo list
    // with `todo check 1`
    // // Alternatively the option to type some of the text from the todo list
    // // item should also be possible
    // The program informs him about the list item he just checked
    //
    // assert ... "[x]" in output
    // assert ... "Refactor code" in output

    // When displaying the list again, the item that was checked is displayed
    // at the bottom, separated by a blank line. It can also be seen, that it's
    // done by the [x] accompanying it.
    // // in color terminals it could even be a different color later on
    // Drinking coffee with Greg is now item number 1

    // Mark notices, that having a coffe meetup in his project's todo list
    // is a bit inappropriate, when after all he wants to show his boss this
    // nice, new todo utility program, that he found.
    // He decides to remove it from the list (`todo remove 1`)

    // He decides, that he doesn't want the todo list after all.
    // Mark runs `todo destroy` and all list items are deleted along with the
    // .todo.toml file
    Err(Box::<dyn std::error::Error>::from(
        "Finish the test!".to_string(),
    ))
    // Ok(())
}
