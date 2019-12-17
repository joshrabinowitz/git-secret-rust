use ini::Ini;

use crate::{consts, types};

/// Returns the existing file system database.
fn open() -> types::Result<Ini> {
  Ok(Ini::load_from_file(consts::SecretDir::PathsMapping.path())?)
}

/// Appends a sequence of files to the file system database.
/// We store our files under `[git-secret]` section.
/// The format is: `key=` for files that are not hidden.
/// And `key=$hash` when files are hidden.
pub fn append_files(files: &Vec<&str>) -> types::Result<()> {
  let mut fsdb = open()?;

  for file in files {
    fsdb
      .with_section(Some(String::from("git-secret")))
      .set(&file[..], "");
  }

  fsdb.write_to_file(consts::SecretDir::PathsMapping.path())?;
  Ok(())
}
