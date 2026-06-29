use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

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
fn stdin_assign() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("mangle")?
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let cmd_stdin = cmd.stdin.as_mut().unwrap();
    cmd_stdin.write_all(b"cat is fat\n")?;
    let _ = cmd_stdin;

    let output = cmd.wait_with_output()?;
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "Reading from stdin...\n5\n"
    );

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

#[test]
fn goto_skips_fallthrough() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("mangle")?;
    cmd.arg("tests/program.ga");
    cmd.assert()
        .success()
        .stdout(predicate::str::is_match(r"^3\n$").unwrap());

    Ok(())
}

#[test]
fn print_op_outputs_to_stdout() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("mangle")?
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    let cmd_stdin = cmd.stdin.as_mut().unwrap();
    // output (6) prints 6; since an output op ran the cli does not echo.
    cmd_stdin.write_all(b"output things\n")?;
    let _ = cmd_stdin;
    let output = cmd.wait_with_output()?;
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "Reading from stdin...\n6\n"
    );
    Ok(())
}

#[test]
fn custom_field_separator() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("mangle")?
        .args(["-F", "-", "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    let cmd_stdin = cmd.stdin.as_mut().unwrap();
    cmd_stdin.write_all(b"add-cat-fat\n")?;
    let _ = cmd_stdin;
    let output = cmd.wait_with_output()?;
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "Reading from stdin...\n6\n"
    );
    Ok(())
}

#[test]
fn fizzbuzz_ascends_one_to_fifteen() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("mangle")?;
    cmd.arg("examples/fizzbuzz.ga");
    let expected = "1\n2\nFizz\n4\nBuzz\nFizz\n7\n8\nFizz\nBuzz\n11\nFizz\n13\n14\nFizzBuzz\n";
    cmd.assert()
        .success()
        .stdout(predicate::eq(expected));
    Ok(())
}
