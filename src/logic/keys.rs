use gpgme::{Context, Data, ExportMode, ImportResult, Protocol};

use crate::{consts, types};

pub fn export(key_ids: Vec<&str>) -> types::Result<Vec<u8>> {
  // TODO: we can lately support different protocols based on some setting.
  let mut gpg = Context::from_protocol(Protocol::OpenPgp)?;
  gpg.set_armor(true);

  let mut key_iter = gpg.find_keys(key_ids)?;
  let found_keys: Vec<_> = key_iter.by_ref().collect::<Result<_, _>>()?;
  if key_iter.finish()?.is_truncated() {
    // TODO: improve error message:
    Err("key listing unexpectedly truncated")?;
  }

  let mut output = Vec::new();
  gpg.export_keys(&found_keys, ExportMode::empty(), &mut output)?;
  Ok(output)
}

pub fn import(key_contents: Vec<u8>) -> types::Result<ImportResult> {
  let mut git_secret_keyring = Context::from_protocol(Protocol::OpenPgp)?;
  git_secret_keyring.set_engine_home_dir(
    consts::SecretDir::Base
      .path() // TODO: create .string() method in consts
      .to_string_lossy()
      .into_owned(),
  )?;

  let mut data = Data::from_reader(&key_contents[..])?;
  Ok(git_secret_keyring.import(&mut data)?)
}
