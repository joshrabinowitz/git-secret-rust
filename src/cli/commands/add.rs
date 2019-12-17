use std::path;

use clap::ArgMatches;

use git_secret::{logic, types};
use git_secret::errors::ToPathIOErr;

use git2::Repository;

pub fn run(args: &ArgMatches) -> types::Result<String> {
  let files: Vec<&str> =
    args.values_of("file").expect("required argument").collect();

  if let Ok(repo) = Repository::open_from_env() {
    println!("{:?}", repo.index().unwrap().get_path(path::Path::new(files[0]), 0).unwrap().path);
  }

  // Returns an error if any of the files are missing:
  files
    .iter()
    .map(|file| {
      // TODO: check that file is inside the main dir.
      // TODO: check that file is not cached:
      // https://docs.rs/git2/0.10.1/git2/struct.Index.html#method.get_path
      // TODO: prevent duplicates in fsdb
      let path = path::Path::new(file);
      logic::files::ensure_file_exists(&path).with_path(&path)
    })
    .collect::<types::Result<path::PathBuf>>()?;

  // TODO: add files to .gitignore
  // https://docs.rs/git2/0.10.1/git2/struct.Repository.html#method.add_ignore_rule
  // https://docs.rs/git2/0.10.1/git2/struct.Repository.html#method.is_path_ignored

  // Appends files to the fsdb:
  logic::fsdb::append_files(&files)?;
  Ok(format!("added {}", files.join(", ")))
}
