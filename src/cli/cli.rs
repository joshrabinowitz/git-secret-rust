use clap::{
  crate_description, crate_name, crate_version, App, AppSettings, Arg,
  SubCommand,
};

pub fn build_cli() -> App<'static, 'static> {
  App::new(crate_name!())
    .version(crate_version!())
    .about(crate_description!())
    .global_setting(AppSettings::ColorAuto)
    .global_setting(AppSettings::ColoredHelp)
    .global_setting(AppSettings::DeriveDisplayOrder)
    .global_setting(AppSettings::UnifiedHelpMessage)
    .global_setting(AppSettings::VersionlessSubcommands)
    .global_setting(AppSettings::DontCollapseArgsInUsage)
    .setting(AppSettings::SubcommandRequiredElseHelp)
    .arg(
      Arg::with_name("dry_run")
        .takes_value(false)
        .help("Performs the dry run and does not actually do anything"),
    )
    .subcommand(SubCommand::with_name("init").about(
      "Creates the .gitsecret directory and contents needed for git-secret",
    ))
    .subcommand(
      SubCommand::with_name("tell")
        .about("Adds access for user by importing public key via email")
        .arg(
          Arg::with_name("email")
            .required(true)
            .multiple(true)
            .help("Users to give access to"),
        )
        .arg(
          Arg::with_name("me")
            .short("m")
            .help("Use my email from git"),
        ),
    )
    .subcommand(
      SubCommand::with_name("add")
        .about("Adds file to be hidden later, also adds file to .gitignore")
        .arg(
          Arg::with_name("file")
            .required(true)
            .multiple(true)
            .help("Files to hide later"),
        ),
    )
}
