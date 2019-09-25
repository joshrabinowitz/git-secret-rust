use std::path;

use clap::ArgMatches;

use git_secret::{logic, types};
use git_secret::errors::ToPathIOErr;

pub fn run(args: &ArgMatches) -> types::Result<String> {
  let files: Vec<&str> =
    args.values_of("file").expect("required argument").collect();

  // Exists if any of the files are missing:
  files
    .iter()
    .map(|file| {
      let path = path::Path::new(file);
      logic::files::ensure_file_exists(&path).with_path(&path)
    })
    .collect::<types::Result<path::PathBuf>>()?;

  // Appends files to the fsdb:
  logic::fsdb::append_files(files)?;

  Ok(String::from("Added"))
}
