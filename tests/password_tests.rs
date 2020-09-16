use std::process::Command;
use assert_cmd::prelude::*;

#[test]
fn test_program_execution_is_successful() {
    let mut cmd = Command::cargo_bin("password").unwrap();
    cmd.assert().success();
}