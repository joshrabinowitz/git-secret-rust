use std::error::Error;
use std::io;
use std::process;

#[macro_use]
extern crate clap;
use clap::{App, ArgMatches};

mod commands;

fn call_command<F>(command: F, args: &ArgMatches) -> io::Result<()>
where
  F: Fn(&ArgMatches) -> commands::CommandResult,
{
  let result = command(args);
  let command_name = result.name;

  result
    .output
    .map(|success| println!("{} {}", command_name, success))
    .map_err(|error| {
      println!("{} failed: {}", command_name, error.description());
      error
    })
}

fn exit(command_result: io::Result<()>) -> ! {
  process::exit(match command_result {
    Ok(_) => 0,
    Err(_) => 1,
  })
}

fn main() -> ! {
  // The YAML file is found relative to the current file,
  // similar to how modules are found.
  let yaml = load_yaml!("cli.yml");
  let matches = App::from_yaml(yaml).get_matches();

  let command_result = match matches.subcommand() {
    ("init", Some(args)) => call_command(commands::init::run, args),
    _ => panic!("Unknown command"), // this cannot realistically happen!
  };

  exit(command_result)
}
