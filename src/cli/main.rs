use std::process;

use clap::ArgMatches;

use git_secret::logic;
use git_secret::types;
mod cli;
mod commands;

fn log_abort<T>(result: types::Result<T>) -> types::Result<T> {
  result.map_err(|failure| {
    eprintln!("git-secret: abort: {}", failure);
    failure
  })
}

fn call_command<F>(command: F, args: &ArgMatches) -> types::Result<String>
where
  F: Fn(&ArgMatches) -> types::Result<String>,
{
  log_abort(command(args).map(|success| {
    println!("git-secret: {}", success);
    success
  }))
}

fn dispatch_command() -> types::Result<String> {
  // Peforms sanity checks before executing any commands:
  log_abort(logic::checks::inside_repo())?;
  log_abort(logic::checks::clean_secring())?;

  // Building CLI parser:
  let matches = cli::build_cli().get_matches();

  match matches.subcommand() {
    ("init", Some(args)) => call_command(commands::init::run, args),
    ("tell", Some(args)) => call_command(commands::tell::run, args),
    ("add", Some(args)) => call_command(commands::add::run, args),
    _ => unreachable!("Unknown command"), // this cannot realistically happen!
  }
}

fn main() -> ! {
  process::exit(match dispatch_command() {
    Ok(_) => 0,
    Err(_) => 1,
  });
}
