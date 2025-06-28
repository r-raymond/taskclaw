#![allow(clippy::needless_borrows_for_generic_args)]

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("claw").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("A simple CLI task tracking tool"));
}

#[test]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("claw").unwrap();
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("claw"));
}

#[test]
fn test_add_task() {
    let temp_dir = tempdir().unwrap();
    let mut cmd = Command::cargo_bin("claw").unwrap();
    cmd.env("HOME", temp_dir.path());
    cmd.args(&["add", "Test task"]);
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Added task: Test task"));
}

#[test]
fn test_list_empty_tasks() {
    let temp_dir = tempdir().unwrap();
    let mut cmd = Command::cargo_bin("claw").unwrap();
    cmd.env("HOME", temp_dir.path());
    cmd.arg("list");
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("No tasks found"));
}

#[test]
fn test_add_and_list_task() {
    let temp_dir = tempdir().unwrap();

    // Add a task
    let mut cmd = Command::cargo_bin("claw").unwrap();
    cmd.env("HOME", temp_dir.path());
    cmd.args(&["add", "Test task"]);
    cmd.assert().success();

    // List tasks
    let mut cmd = Command::cargo_bin("claw").unwrap();
    cmd.env("HOME", temp_dir.path());
    cmd.arg("list");
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Test task"))
        .stdout(predicates::str::contains("○ [0]"));
}

#[test]
fn test_complete_task() {
    let temp_dir = tempdir().unwrap();

    // Add a task
    let mut cmd = Command::cargo_bin("claw").unwrap();
    cmd.env("HOME", temp_dir.path());
    cmd.args(&["add", "Test task"]);
    cmd.assert().success();

    // Complete the task
    let mut cmd = Command::cargo_bin("claw").unwrap();
    cmd.env("HOME", temp_dir.path());
    cmd.args(&["complete", "0"]);
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Completed task 0"));

    // Verify task is completed
    let mut cmd = Command::cargo_bin("claw").unwrap();
    cmd.env("HOME", temp_dir.path());
    cmd.arg("list");
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("✓ [0]"));
}

#[test]
fn test_remove_task() {
    let temp_dir = tempdir().unwrap();

    // Add a task
    let mut cmd = Command::cargo_bin("claw").unwrap();
    cmd.env("HOME", temp_dir.path());
    cmd.args(&["add", "Test task"]);
    cmd.assert().success();

    // Remove the task
    let mut cmd = Command::cargo_bin("claw").unwrap();
    cmd.env("HOME", temp_dir.path());
    cmd.args(&["remove", "0"]);
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Removed task 0"));

    // Verify task is removed
    let mut cmd = Command::cargo_bin("claw").unwrap();
    cmd.env("HOME", temp_dir.path());
    cmd.arg("list");
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("No tasks found"));
}

#[test]
fn test_complete_nonexistent_task() {
    let temp_dir = tempdir().unwrap();
    let mut cmd = Command::cargo_bin("claw").unwrap();
    cmd.env("HOME", temp_dir.path());
    cmd.args(&["complete", "999"]);
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Task 999 not found"));
}

#[test]
fn test_remove_nonexistent_task() {
    let temp_dir = tempdir().unwrap();
    let mut cmd = Command::cargo_bin("claw").unwrap();
    cmd.env("HOME", temp_dir.path());
    cmd.args(&["remove", "999"]);
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Task 999 not found"));
}

#[test]
fn test_completions_bash() {
    let mut cmd = Command::cargo_bin("claw").unwrap();
    cmd.args(&["completions", "bash"]);
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("_claw()"));
}

#[test]
fn test_completions_zsh() {
    let mut cmd = Command::cargo_bin("claw").unwrap();
    cmd.args(&["completions", "zsh"]);
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("#compdef claw"));
}

#[test]
fn test_completions_fish() {
    let mut cmd = Command::cargo_bin("claw").unwrap();
    cmd.args(&["completions", "fish"]);
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("function __fish_claw"));
}

#[test]
fn test_completions_invalid_shell() {
    let mut cmd = Command::cargo_bin("claw").unwrap();
    cmd.args(&["completions", "invalid"]);
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("Unsupported shell: invalid"));
}

#[test]
fn test_multiple_tasks_workflow() {
    let temp_dir = tempdir().unwrap();

    // Add multiple tasks
    for i in 1..=3 {
        let mut cmd = Command::cargo_bin("claw").unwrap();
        cmd.env("HOME", temp_dir.path());
        cmd.args(&["add", &format!("Task {}", i)]);
        cmd.assert().success();
    }

    // List all tasks
    let mut cmd = Command::cargo_bin("claw").unwrap();
    cmd.env("HOME", temp_dir.path());
    cmd.arg("list");
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Task 1"))
        .stdout(predicates::str::contains("Task 2"))
        .stdout(predicates::str::contains("Task 3"));

    // Complete middle task
    let mut cmd = Command::cargo_bin("claw").unwrap();
    cmd.env("HOME", temp_dir.path());
    cmd.args(&["complete", "1"]);
    cmd.assert().success();

    // Remove first task
    let mut cmd = Command::cargo_bin("claw").unwrap();
    cmd.env("HOME", temp_dir.path());
    cmd.args(&["remove", "0"]);
    cmd.assert().success();

    // Verify final state
    let mut cmd = Command::cargo_bin("claw").unwrap();
    cmd.env("HOME", temp_dir.path());
    cmd.arg("list");
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("✓ [1] Task 2"))
        .stdout(predicates::str::contains("○ [2] Task 3"))
        .stdout(predicates::str::contains("Task 1").not());
}
