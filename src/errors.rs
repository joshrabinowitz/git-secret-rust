extern crate custom_error;

use custom_error::custom_error;

use std::io;
use std::path::{Path, PathBuf};

#[macro_export]
custom_error! {
  pub GitSecretError

  IO {
    source: io::Error, // We also wrap the source error here.
    path: PathBuf
  } = @{format!("{}: {}", path.display(), source)},

  Simple {
    reason: String
  } = @{format!("{}", reason)},
}

pub trait ToPathIOErr<T> {
  fn with_path(self: Self, path: &Path) -> Result<T, GitSecretError>;
}

impl<T> ToPathIOErr<T> for io::Result<T> {
  fn with_path(self: Self, path: &Path) -> Result<T, GitSecretError> {
    self.map_err(|e| GitSecretError::IO {
      source: e,
      path: path.to_path_buf(),
    })
  }
}
