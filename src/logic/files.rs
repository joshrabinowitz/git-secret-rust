use std::fs;
use std::io;
use std::path;

pub fn ensure_file_exists(file: &path::Path) -> io::Result<&path::Path> {
  fs::metadata(file)?;
  Ok(file)
}
