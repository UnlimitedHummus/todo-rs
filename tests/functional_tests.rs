use assert_cmd::Command;

#[test]
fn test_programm_launches() {
    let mut cmd = Command::cargo_bin("todo-rs").unwrap();
    cmd.assert().success();
}

