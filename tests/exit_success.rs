#[test]
fn exit_success() {
    let mut child = std::process::Command::new(env!("CARGO_BIN_EXE_ewc"))
        .arg("0")
        .spawn()
        .unwrap();

    assert!(child.wait().unwrap().success())
}
