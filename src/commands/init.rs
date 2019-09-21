use clap::ArgMatches;
use std::fs;
use std::io;
use std::path;

use super::CommandResult;

const SECRETS_DIR: &str = ".gitsecrets";
const SECRETS_DIR_KEYS: &str = "keys";
const SECRETS_DIR_PATHS: &str = "paths";
const SECRETS_DIR_PATHS_MAPPING: &str = "mapping.cfg";

fn execute(_args: &ArgMatches) -> io::Result<String> {
  let path = path::Path::new(SECRETS_DIR);

  // Creating file structure:
  fs::create_dir(path)?;
  fs::create_dir(path.join(SECRETS_DIR_KEYS))?;
  fs::create_dir(path.join(SECRETS_DIR_PATHS))?;

  // Creating required files:
  fs::File::create(
    path.join(SECRETS_DIR_KEYS).join(SECRETS_DIR_PATHS_MAPPING),
  )?;

  // TODO: gitignore_add_pattern "$random_seed_file"
  // TODO: gitignore_add_pattern "!*$SECRETS_EXTENSION"

  Ok(format!("created: {}", path.display()))
}

pub fn run(args: &ArgMatches) -> CommandResult {
  CommandResult {
    name: "init".to_string(),
    output: execute(args),
  }
}
