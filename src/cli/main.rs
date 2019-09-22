use std::process;

use clap::ArgMatches;

use git_secret::errors;
use git_secret::logic;
mod cli;
mod commands;

fn log_abort<T>(
  result: Result<T, errors::GitSecretError>,
) -> Result<T, errors::GitSecretError> {
  result.map_err(|failure| {
    eprintln!("git-secret: abort: {}", failure);
    failure
  })
}

fn call_command<F>(
  command: F,
  args: &ArgMatches,
) -> Result<String, errors::GitSecretError>
where
  F: Fn(&ArgMatches) -> Result<String, errors::GitSecretError>,
{
  log_abort(command(args).map(|success| {
    println!("{}", success);
    success
  }))
}

fn dispatch_command() -> Result<String, errors::GitSecretError> {
  // Peforms sanity checks before executing any commands:
  log_abort(logic::checks::inside_repo())?;
  log_abort(logic::checks::clean_secring())?;

  // Building CLI parser:
  let matches = cli::build_cli().get_matches();

  match matches.subcommand() {
    ("init", Some(args)) => call_command(commands::init::run, args),
    _ => unreachable!("Unknown command"), // this cannot realistically happen!
  }
}

fn main() -> ! {
  process::exit(match dispatch_command() {
    Ok(_) => 0,
    Err(_) => 1,
  });
}
