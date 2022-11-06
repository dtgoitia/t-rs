use dialoguer::{console::Term, theme::ColorfulTheme, FuzzySelect};

use crate::config::AppConfig;

pub fn status(config: &AppConfig) {
    println!("Showing timer status! token {}", config.api_token.as_str());
    let items = vec!["Item 1", "Item 2"];
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .expect("TODO: handle this error");

    match selection {
        Some(index) => println!("User selected item: {}", items[index]),
        None => println!("USer did not select anything"),
    };
}

pub fn start(config: &AppConfig) {
    println!("Timer started! token {}", config.api_token.as_str());
}

pub fn stop(config: &AppConfig) {
    println!("Timer stopped! token {}", config.api_token.as_str());
}
