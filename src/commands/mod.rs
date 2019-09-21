pub mod init;

use std::io;

pub struct CommandResult {
  pub name: String,
  pub output: io::Result<String>,
}
