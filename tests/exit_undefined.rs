use std::ffi::OsStr;
use std::process::Stdio;

#[yare::parameterized(
    letter = { OsStr::new("a") },
    empty = { OsStr::new("") },
    to_large = { OsStr::new("2147483648") },
    to_small = { OsStr::new("-2147483649") },
)]
fn exit_undefined_argument_invalid(arg: &OsStr) {
    let child = std::process::Command::new(env!("CARGO_BIN_EXE_ewc"))
        .arg(arg)
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let output = child.wait_with_output().unwrap();

    let success = output.status.success();
    let stderr = String::from_utf8(output.stderr).unwrap();

    let expected = format!(
        "Argument '{:?}' could not be parsed as a valid integer (i32)",
        arg
    )
    .replace("\"", "");

    assert!(!stderr.is_empty());
    assert!(stderr.starts_with(&expected));

    assert!(!success);
}

#[test]
fn exit_undefined_no_args() {
    let child = std::process::Command::new(env!("CARGO_BIN_EXE_ewc"))
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let output = child.wait_with_output().unwrap();

    let success = output.status.success();
    let stderr = String::from_utf8(output.stderr).unwrap();

    let expected = "Expected 1 argument (the exit code), but 0 were given";

    assert!(!stderr.is_empty());
    assert!(stderr.starts_with(&expected));

    assert!(!success);
}

#[test]
fn exit_undefined_too_many_args() {
    let child = std::process::Command::new(env!("CARGO_BIN_EXE_ewc"))
        .stderr(Stdio::piped())
        .args(&["1", "2"])
        .spawn()
        .unwrap();

    let output = child.wait_with_output().unwrap();

    let success = output.status.success();
    let stderr = String::from_utf8(output.stderr).unwrap();

    let expected = "Expected 1 argument (the exit code), but 2 were given";

    assert!(!stderr.is_empty());
    assert!(stderr.starts_with(&expected));

    assert!(!success);
}
