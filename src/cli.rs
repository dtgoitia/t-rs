use crate::config;
use crate::toggl;
use clap::Command;

pub fn cli(config: &config::AppConfig) {
    println!("{}", config.projects[0].name);
    let matches = Command::new("t")
        .subcommand(Command::new("start").about("start timer"))
        .subcommand(Command::new("stop").about("stop timer"))
        .subcommand(Command::new("status").about("show current toggl status (running, stopped...)"))
        .get_matches();

    match matches.subcommand() {
        Some(("status", _)) => return toggl::status(&config),
        Some(("start", _)) => return toggl::start(&config),
        Some(("stop", _)) => return toggl::stop(&config),
        _ => return toggl::start(&config),
    };
}
