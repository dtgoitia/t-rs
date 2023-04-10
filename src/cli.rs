use crate::config;
use crate::domain;
use clap::Command;

pub fn cli(config: &config::AppConfig) {
    let matches = Command::new("t")
        .subcommand(Command::new("start").about("start timer"))
        .subcommand(Command::new("stop").about("stop timer"))
        .subcommand(Command::new("status").about("show current toggl status (running, stopped...)"))
        .subcommand(Command::new("swap").about("swap ongoing timer without changing start time"))
        .subcommand(Command::new("shift").about("shift start time of ongoing timer"))
        .get_matches();

    match matches.subcommand() {
        Some(("status", _)) => domain::show_toggl_status(&config),
        Some(("start", _)) => domain::start_toggl_timer(&config),
        Some(("stop", _)) => domain::stop_toggl_timer(&config),
        Some(("swap", _)) => domain::swap_current_toggl_timer(&config),
        Some(("shift", _)) => domain::shift_current_toggl_timer(&config),
        _ => domain::start_toggl_timer(&config),
    };
}
