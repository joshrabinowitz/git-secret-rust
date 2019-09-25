use std::path;

use ini::Ini;

use crate::{consts, types};

fn open() -> types::Result<Ini> {
  Ok(Ini::load_from_file(consts::SecretDir::PathsMapping.path())?)
}

pub fn append_files(files: Vec<&str>) -> types::Result<()> {
  let mut fsdb = open()?;

  for file in &files {
    fsdb
      .with_section(Some(String::from("git-secret")))
      .set(&file[..], "");
  }

  fsdb.write_to_file(consts::SecretDir::PathsMapping.path())?;
  Ok(())
}
