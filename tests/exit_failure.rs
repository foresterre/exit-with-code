#[test]
fn exit_failure() {
    let child = std::process::Command::new(env!("CARGO_BIN_EXE_ewc"))
        .arg("1")
        .spawn()
        .unwrap();

    let output = child.wait_with_output().unwrap();

    assert!(output.stderr.is_empty());
    assert!(!output.status.success());
}
