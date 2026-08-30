use std::env;
use std::process::ExitCode;

//this is helper function solely to show the unit tests work.
fn help() -> &'static str {
    HELP
}

const HELP: &str = "\
A small agent

Usage: mu [OPTIONS]

Options:
  -h, --help  Print help
";

fn main() -> ExitCode {
    let argument = env::args().nth(1);
    match argument.as_deref() {
        Some("-h") | Some("--help") => {
            print!("{}", help());
            ExitCode::SUCCESS
        }
        None => {
            eprintln!("error: no argument provided");
            eprintln!();
            eprintln!("Usage: mu [OPTIONS]");
            eprintln!();
            eprintln!("For more information, try '--help'.");
            ExitCode::from(2)
        }
        Some(unknown) => {
            eprintln!("error: unexpected argument '{unknown}'");
            eprintln!();
            eprintln!("Usage: mu [OPTIONS]");
            eprintln!();
            eprintln!("For more information, try '--help'.");
            ExitCode::from(2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world_returns_hello_world() {
        assert_eq!(help(), HELP)
    }
}
