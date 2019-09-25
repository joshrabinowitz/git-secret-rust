use std::fmt::Write;

use clap::ArgMatches;

use gpgme;

use git_secret::logic;
use git_secret::types;

pub fn run(args: &ArgMatches) -> types::Result<String> {
  // Export:
  let key_ids: Vec<&str> = args
    .values_of("email")
    .expect("required argument")
    .collect();
  let key_contents = logic::keys::export(key_ids)?;

  // Import:
  let import_result = logic::keys::import(key_contents)?;
  // TODO: return error if not all keys are imported
  report_import_result(import_result)
}

/// Reports key import result.
/// It contains all the information about added,
/// changed, and not imported keys.
///
/// Based on gpgme example. See:
/// https://github.com/gpg-rs/gpgme/blob/master/examples/import.rs
///
/// Format is skipped for report to be human-readable.
#[rustfmt::skip]
fn report_import_result(
  result: gpgme::ImportResult,
) -> types::Result<String> {
  let mut report = String::new();

  writeln!(&mut report, "imported")?;

  for import in result.imports() {
    writeln!(&mut report, "fpr: {}",
      import.fingerprint().unwrap_or("[none]"),
    )?;
  }

  writeln!(&mut report, "")?;

  writeln!(&mut report, "key import summary:")?;
  writeln!(&mut report, "        considered: {}", result.considered())?;
  writeln!(&mut report, "        no user id: {}", result.without_user_id())?;
  writeln!(&mut report, "          imported: {}", result.imported())?;
  writeln!(&mut report, "      imported rsa: {}", result.imported_rsa())?;
  writeln!(&mut report, "         unchanged: {}", result.unchanged())?;
  writeln!(&mut report, "      new user ids: {}", result.new_user_ids())?;
  writeln!(&mut report, "       new subkeys: {}", result.new_subkeys())?;
  writeln!(&mut report, "    new signatures: {}", result.new_signatures())?;
  writeln!(&mut report, "   new revocations: {}", result.new_revocations())?;
  writeln!(&mut report, "       secret read: {}", result.secret_considered())?;
  writeln!(&mut report, "   secret imported: {}", result.secret_imported())?;
  writeln!(&mut report, "  secret unchanged: {}", result.secret_unchanged())?;
  writeln!(&mut report, "      not imported: {}", result.not_imported())?;

  Ok(report)
}
