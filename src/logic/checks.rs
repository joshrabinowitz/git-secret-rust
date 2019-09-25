use std::fs;

use git2::Repository;

use crate::consts;
use crate::types;

pub fn inside_repo() -> types::Result<()> {
  match Repository::open_from_env() {
    Ok(_repo) => Ok(()),
    Err(_) => Err("not in dir with git repo")?,
  }
}

// TODO: check that `.gitsecret/private-keys-v1.d` folder is empty
pub fn clean_secring() -> types::Result<()> {
  let metadata = fs::metadata(consts::SecretDir::KeysSecring.path());

  if let Ok(file) = metadata {
    if !file.is_file() || file.len() > 0 {
      Err("it seems that someone has imported a secret key.")?;
    }
  };

  Ok(())
}
