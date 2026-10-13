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

#[test]
fn mu_runs_with_any_option_order() {
    let mut pairs = [
        ["--provider", "fake"],
        ["--model", "fake"],
        ["-p", "write a binary search in python"],
    ];

    for_each_permutation(&mut pairs, &mut |permutation| {
        let args: Vec<&str> = permutation.iter().flatten().copied().collect();
        let output = run_mu(&args);

        let stdout = String::from_utf8(output.stdout).unwrap();
        assert_eq!(output.status.code(), Some(0));
        assert_eq!(stdout, "FakeProvider answer\n");
        assert!(output.stderr.is_empty());
    });
}

fn for_each_permutation<T>(items: &mut [T], visit: &mut impl FnMut(&[T])) {
    fn generate<T>(items: &mut [T], position: usize, visit: &mut impl FnMut(&[T])) {
        if position == items.len() {
            visit(items);
            return;
        }

        for index in position..items.len() {
            items.swap(position, index);
            generate(items, position + 1, visit);
            items.swap(position, index);
        }
    }
    generate(items, 0, visit);
}

#[test]
fn mu_rejects_missing_provider() {
    let mut pairs = [
        ["--model", "fake"],
        ["-p", "write a binary search in python"],
    ];

    for_each_permutation(&mut pairs, &mut |permutation| {
        let args: Vec<&str> = permutation.iter().flatten().copied().collect();
        let output = run_mu(&args);

        let stdout = String::from_utf8(output.stdout).unwrap();
        assert_eq!(output.status.code(), Some(2));
        assert!(stdout.is_empty());
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(stderr.contains("--provider is required"));
    });
}

#[test]
fn mu_rejects_missing_model() {
    let mut pairs = [
        ["--provider", "fake"],
        ["-p", "write a binary search in python"],
    ];

    for_each_permutation(&mut pairs, &mut |permutation| {
        let args: Vec<&str> = permutation.iter().flatten().copied().collect();
        let output = run_mu(&args);

        let stdout = String::from_utf8(output.stdout).unwrap();
        assert_eq!(output.status.code(), Some(2));
        assert!(stdout.is_empty());
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(stderr.contains("--model is required"));
    });
}
#[test]
fn mu_rejects_missing_prompt() {
    let mut pairs = [["--provider", "fake"], ["--model", "fake"]];

    for_each_permutation(&mut pairs, &mut |permutation| {
        let args: Vec<&str> = permutation.iter().flatten().copied().collect();
        let output = run_mu(&args);

        let stdout = String::from_utf8(output.stdout).unwrap();
        assert_eq!(output.status.code(), Some(2));
        assert!(stdout.is_empty());
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(stderr.contains("-p is required"));
    });
}

#[test]
fn mu_rejects_unknown_provider() {
    let args = [
        "--provider",
        "unknown",
        "--model",
        "fake",
        "-p",
        "write a binary search in python",
    ];
    let output = run_mu(&args);

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(stdout.is_empty());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("unknown provider 'unknown'"));
}

#[test]
fn mu_rejects_unknown_model() {
    let args = [
        "--provider",
        "fake",
        "--model",
        "unknown",
        "-p",
        "write a binary search in python",
    ];
    let output = run_mu(&args);

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(stdout.is_empty());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("unknown model 'unknown'"));
}

#[test]
fn mu_rejects_provider_without_value() {
    let args = [
        "--provider",
        "--model",
        "fake",
        "-p",
        "write a binary search in python",
    ];
    let output = run_mu(&args);

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(stdout.is_empty());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(!stderr.is_empty());
}

#[test]
fn mu_rejects_model_without_value() {
    let args = [
        "--provider",
        "fake",
        "--model",
        "-p",
        "write a binary search in python",
    ];
    let output = run_mu(&args);

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(stdout.is_empty());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(!stderr.is_empty());
}

#[test]
fn mu_rejects_prompt_without_value() {
    let args = ["--provider", "fake", "--model", "fake", "-p"];
    let output = run_mu(&args);

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(stdout.is_empty());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(!stderr.is_empty());
}

#[test]
fn mu_rejects_unknown_arg() {
    let args = [
        "--unknown",
        "--provider",
        "fake",
        "--model",
        "fake",
        "-p",
        "write a binary search in python",
    ];
    let output = run_mu(&args);

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(stdout.is_empty());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(!stderr.is_empty());
}

#[test]
fn mu_rejects_duplicate_provider() {
    let args = [
        "--provider",
        "fake",
        "--provider",
        "fake",
        "--model",
        "fake",
        "-p",
        "write a binary search in python",
    ];
    let output = run_mu(&args);

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(stdout.is_empty());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(!stderr.is_empty());
}

#[test]
fn mu_rejects_duplicate_model() {
    let args = [
        "--provider",
        "fake",
        "--model",
        "fake",
        "--model",
        "fake",
        "-p",
        "write a binary search in python",
    ];
    let output = run_mu(&args);

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(stdout.is_empty());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(!stderr.is_empty());
}

#[test]
fn mu_rejects_duplicate_prompt() {
    let args = [
        "--provider",
        "fake",
        "--model",
        "fake",
        "-p",
        "write a binary search in python",
        "-p",
        "write a binary search in python",
    ];
    let output = run_mu(&args);

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(stdout.is_empty());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(!stderr.is_empty());
}
