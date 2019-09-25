extern crate custom_error;

use custom_error::custom_error;

use std::fmt;
use std::io;
use std::path::{Path, PathBuf};

use gpgme::data;
use gpgme::error as gpg_error;

// We use this error as the main one accross this app.
custom_error! {
  pub GitSecretError

  Format {
    source: fmt::Error
  } = @{format!("{}", source)},

  IO {
    source: io::Error
  } = @{format!("{}", source)},

  PathIO {
    source: io::Error, // We also wrap the source error here.
    path: PathBuf
  } = @{format!("{}: {}", path.display(), source)},

  GPG {
    source: gpg_error::Error
  } = @{format!("gpg: {}", source)},

  Simple {
    reason: String
  } = @{format!("{}", reason)},
}

/// We need this custom error cast, because `custom_error!` does not support
/// generic error messages. See:
/// https://github.com/lovasoa/custom_error/issues/9
impl<S> From<data::WrappedError<S>> for GitSecretError {
  fn from(error: data::WrappedError<S>) -> Self {
    GitSecretError::GPG {
      source: error.error(),
    }
  }
}

/// The simplest case: we create error from `&str` description.
impl From<&str> for GitSecretError {
  fn from(error: &str) -> Self {
    GitSecretError::Simple {
      reason: String::from(error),
    }
  }
}

/// This trait is used to give extra context (path)
/// to the builtin `io::Result` type.
pub trait ToPathIOErr<T> {
  fn with_path(self: Self, path: &Path) -> Result<T, GitSecretError>;
}

impl<T> ToPathIOErr<T> for io::Result<T> {
  fn with_path(self: Self, path: &Path) -> Result<T, GitSecretError> {
    self.map_err(|e| GitSecretError::PathIO {
      source: e,
      path: path.to_path_buf(),
    })
  }
}
