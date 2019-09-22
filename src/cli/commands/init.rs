use clap::ArgMatches;
use std::fs;

use git_secret::errors::ToPathIOErr;
use git_secret::{consts, errors};

pub fn run(_args: &ArgMatches) -> Result<String, errors::GitSecretError> {
  // Creating file structure:
  let paths = vec![
    consts::Paths::SecretsDir.path(),
    consts::Paths::SecretsDirKeys.path(),
    consts::Paths::SecretsDirPaths.path(),
  ];

  for path in paths {
    fs::create_dir(&path).with_path(&path)?;
  }

  // Creating required files:
  let files = vec![consts::Paths::SecretsDirPathsMapping.path()];

  for file in files {
    fs::File::create(&file).with_path(&file)?;
  }

  // TODO: gitignore_add_pattern "$random_seed_file"
  // TODO: gitignore_add_pattern "!*$SECRETS_EXTENSION"

  Ok(format!(
    "init created: {}",
    consts::Paths::SecretsDir.path().display()
  ))
}
