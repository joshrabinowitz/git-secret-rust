use std::fs;

use clap::ArgMatches;

use git_secret::errors::ToPathIOErr;
use git_secret::{consts, types};

pub fn run(_args: &ArgMatches) -> types::Result<String> {
  // Creating file structure:
  let paths = vec![
    consts::SecretDir::Base.path(),
    consts::SecretDir::Keys.path(),
    consts::SecretDir::Paths.path(),
  ];

  for path in paths {
    fs::create_dir(&path).with_path(&path)?;
  }

  // Creating required files:
  let files = vec![consts::SecretDir::PathsMapping.path()];

  for file in files {
    fs::File::create(&file).with_path(&file)?;
  }

  // TODO: gitignore_add_pattern "$random_seed_file"
  // TODO: gitignore_add_pattern "!*$SECRETS_EXTENSION"

  Ok(format!(
    "init created: {}",
    consts::SecretDir::Base.path().display()
  ))
}
