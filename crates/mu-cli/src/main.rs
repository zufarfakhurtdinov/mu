mod providers;

use mu_agent::{Context, provider::Provider};
use providers::ModelProvider;
use std::env;
use std::process::ExitCode;

//this is helper function solely to show the unit tests work.
fn help() -> &'static str {
    HELP
}

macro_rules! usage_help {
    () => {
        "Usage: mu --provider PROVIDER --model MODEL -p PROMPT"
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

struct CliArgs {
    provider: String,
    model: String,
    prompt: String,
}

enum CliCommand {
    Help,
    Run(CliArgs),
}

fn parse_args(mut args: impl Iterator<Item = String>) -> Result<CliCommand, String> {
    let mut provider = None;
    let mut model = None;
    let mut prompt = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                return Ok(CliCommand::Help);
            }
            "--provider" => {
                if provider.is_some() {
                    return Err("--provider may only be specified once".to_owned());
                }
                let value = args.next().ok_or("--provider requires a value")?;
                provider = Some(value);
            }
            "--model" => {
                if model.is_some() {
                    return Err("--model may only be specified once".to_owned());
                }
                let value = args.next().ok_or("--model requires a value")?;
                model = Some(value);
            }
            "-p" => {
                if prompt.is_some() {
                    return Err("-p may only be specified once".to_owned());
                }
                let value = args.next().ok_or("-p requires a value")?;
                prompt = Some(value);
            }
            unknown => {
                return Err(format!("unexpected argument : '{unknown}'"));
            }
        }
    }
    Ok(CliCommand::Run(CliArgs {
        provider: provider.ok_or("--provider is required")?,
        model: model.ok_or("--model is required")?,
        prompt: prompt.ok_or("-p is required")?,
    }))
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {
    let args = match parse_args(env::args().skip(1)) {
        Ok(CliCommand::Help) => {
            print!("{}", help());
            return ExitCode::SUCCESS;
        }
        Ok(CliCommand::Run(args)) => args,
        Err(error) => {
            eprintln!("error: '{error}'");
            eprintln!();
            print_usage_hint();
            return ExitCode::from(2);
        }
    };

    let context = Context {
        system_prompt: None,
        messages: vec![args.prompt.to_owned()],
        tools: None,
    };
    let provider = match ModelProvider::from_id(args.provider.as_str()) {
        Ok(provider) => provider,
        Err(error) => {
            eprintln!("error: {error}");
            return ExitCode::from(2);
        }
    };
    let Some(model) = provider.find_model(&args.model) else {
        eprintln!("error: unknown model '{}'", args.model);
        return ExitCode::from(2);
    };
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

fn print_usage_hint() {
    eprintln!(usage_help!());
    eprintln!();
    eprintln!("For more information, try '--help'.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mu_help_prints_help() {
        assert_eq!(help(), HELP)
    }
}
