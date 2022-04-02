use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::{Command, Stdio}; // Run programs
use std::io::Write;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("mangle")?;
    cmd.arg("nonexisting.ga");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Cannot read file at"));

    Ok(())
}

#[test]
fn stdin() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("mangle")?
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let cmd_stdin = cmd.stdin.as_mut().unwrap();
    cmd_stdin.write_all(b"cat is fat\n")?;
    // Close stdin to finish and avoid indefinite blocking
    drop(cmd_stdin);
    
    let output = cmd.wait_with_output()?;
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "5\nCTRL-D exiting...\n");

    Ok(())
}

#[test]
fn read_file_cat() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("mangle")?;
    cmd.arg("tests/cat.ga");
    cmd.assert()
        .success()
        .stdout(predicate::str::is_match(r"^5\n$").unwrap());

    Ok(())
}
