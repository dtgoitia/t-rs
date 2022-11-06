use std::env;
use std::path::Path;
use std::process;

mod cli;
mod config;
mod domain;
mod toggl;

fn main() {
    let home = match env::var("HOME") {
        Ok(value) => value,
        Err(error) => return exit_with_error(error.to_string()),
    };

    let dotfiles_dir = Path::new(&home).join(".config/t");

    let config_path = dotfiles_dir.join("config.jsonc");
    let credentials_path = dotfiles_dir.join("credentials.jsonc");

    let config = config::load(&config_path, &credentials_path);

    cli::cli(&config);

    return;
}

fn exit_with_error(message: String) {
    println!("{}", message);
    process::exit(-1);
}
