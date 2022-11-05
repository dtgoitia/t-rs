use std::path::Path;

mod config;

fn main() {
    // TODO: how to do Path.expanduser()
    let user_home = Path::new("/home/dtg");

    let dotfiles_dir = user_home.join(".config/t");

    let config_path = dotfiles_dir.join("config.jsonc");
    let credentials_path = dotfiles_dir.join("credentials.jsonc");
    let config = config::load(&config_path, &credentials_path);

    /*
    TODO: algorithm t
    */
    println!("{}", config.projects[0].name);
    return;
}
