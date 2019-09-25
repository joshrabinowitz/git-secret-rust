use std::result;

use crate::errors;

/// The result type we use inside git-secret.
pub type Result<T> = result::Result<T, errors::GitSecretError>;
