use mu_agent::{Context, provider::Provider};
use mu_provider_fake::FakeProvider;
use std::env;
use std::process::ExitCode;

//this is helper function solely to show the unit tests work.
fn help() -> &'static str {
    HELP
}

macro_rules! usage_help {
    () => {
        "Usage: mu -p PROMPT"
    };
}

const HELP: &str = concat!(
    usage_help!(),
    "

A tiny agent.

Options:
  -p PROMPT   Send a prompt to the agent
  -h, --help  Print help
"
);

#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {
    let mut arguments = env::args().skip(1);
    match arguments.next().as_deref() {
        Some("-h") | Some("--help") => {
            print!("{}", help());
            return ExitCode::SUCCESS;
        }
        None => {
            eprintln!("error: no argument provided");
            eprintln!();
            print_usage_hint();
            return ExitCode::from(2);
        }
        Some("-p") => {
            let Some(prompt) = arguments.next() else {
                eprintln!("error: no prompt provided");
                return ExitCode::from(2);
            };
            if arguments.next().is_some() {
                eprintln!("error: expected only prompt argument");
                return ExitCode::from(2);
            }
            let context = Context {
                system_prompt: None,
                messages: vec![prompt.to_owned()],
                tools: None,
            };
            let provider = FakeProvider;
            let model = mu_provider_fake::model();
            match provider.complete(&model, &context).await {
                Ok(answer) => {
                    println!("{answer}");
                    return ExitCode::from(0);
                }
                Err(error) => {
                    eprintln!("error: {error}");
                    return ExitCode::from(1);
                }
            }
        }
        Some(unknown) => {
            eprintln!("error: unexpected argument '{unknown}'");
            eprintln!();
            print_usage_hint();
            return ExitCode::from(2);
        }
    }
}

fn print_usage_hint() {
    eprintln!(usage_help!());
    eprintln!();
    eprintln!("For more information, try '--help'.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world_returns_hello_world() {
        assert_eq!(help(), HELP)
    }
}
