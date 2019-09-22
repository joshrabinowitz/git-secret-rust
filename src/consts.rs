use std::path::{Path, PathBuf};

pub enum Paths {
  // Directories:
  SecretsDir,
  SecretsDirKeys,
  SecretsDirPaths,

  // Files:
  SecretsDirKeysSecring,

  SecretsDirPathsMapping,
}

impl Paths {
  pub fn path(&self) -> PathBuf {
    let base = Path::new(".gitsecret");
    let keys = base.join("keys");
    let paths = base.join("paths");

    match *self {
      Paths::SecretsDir => base.to_owned(),

      Paths::SecretsDirKeys => keys,
      Paths::SecretsDirPaths => paths,

      Paths::SecretsDirKeysSecring => keys.join("secring.gpg"),
      Paths::SecretsDirPathsMapping => paths.join("mapping.cfg"),
    }
  }
}
