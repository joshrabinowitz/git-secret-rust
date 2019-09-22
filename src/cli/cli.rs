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
    .subcommand(
      SubCommand::with_name("init")
        .about("Setups the project structure for git-secret"),
    )
}
