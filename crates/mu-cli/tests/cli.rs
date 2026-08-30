use std::process::{Command, Output};

const MU: &str = env!("CARGO_BIN_EXE_mu");

fn run_mu(arguments: &[&str]) -> Output {
    Command::new(MU)
        .args(arguments)
        .output()
        .expect("failed to start")
}

#[test]
fn mu_help_prints_help() {
    let output = run_mu(&["--help"]);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Usage:"))
}

#[test]
fn mu_minus_h_prints_help() {
    let output = run_mu(&["-h"]);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Usage:"))
}

#[test]
fn mu_without_args_fails() {
    let output = run_mu(&[]);
    assert!(!output.status.success())
}
