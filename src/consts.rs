use std::path::{Path, PathBuf};

pub enum SecretDir {
  // Directories:
  Base,
  Keys,
  Paths,

  // Files:
  KeysSecring,

  PathsMapping,
}

impl SecretDir {
  pub fn path(&self) -> PathBuf {
    let base = Path::new(".gitsecret");
    let keys = base.join("keys");
    let paths = base.join("paths");

    match *self {
      SecretDir::Base => base.to_owned(),

      SecretDir::Keys => keys,
      SecretDir::Paths => paths,

      SecretDir::KeysSecring => keys.join("secring.gpg"),
      SecretDir::PathsMapping => paths.join("mapping.cfg"),
    }
  }

  pub fn string(&self) -> String {
    self.path().to_string_lossy().into_owned()
  }

  // TODO: support `.relative_path()`
  // https://docs.rs/git2/0.10.1/git2/struct.Repository.html#method.workdir
}
