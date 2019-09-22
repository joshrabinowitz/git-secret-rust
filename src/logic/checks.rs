use std::fs;

use git2::Repository;

use crate::consts;
use crate::errors;

pub fn inside_repo() -> Result<(), errors::GitSecretError> {
  match Repository::open(".") {
    Ok(_repo) => Ok(()),
    Err(_) => Err(errors::GitSecretError::Simple {
      reason: "not in dir with git repo".to_string(),
    }),
  }
}

pub fn clean_secring() -> Result<(), errors::GitSecretError> {
  let metadata = fs::metadata(consts::Paths::SecretsDirKeysSecring.path());

  if let Ok(file) = metadata {
    if !file.is_file() || file.len() > 0 {
      return Err(errors::GitSecretError::Simple {
        reason: "it seems that someone has imported a secret key.".to_string(),
      });
    }
  };

  Ok(())
}
