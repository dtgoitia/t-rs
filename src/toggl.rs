use crate::{config::AppConfig, domain::TogglItem};

pub fn start(config: &AppConfig, item: &TogglItem) {
    println!(
        "Starting {} in project {} ...",
        item.entry_name, item.project_id
    );
    println!("Timer started! token {}", config.api_token.as_str());
}

pub fn status(config: &AppConfig) {
    println!("Showing timer status! token {}", config.api_token.as_str());
}

pub fn stop(config: &AppConfig) {
    println!("Timer stopped! token {}", config.api_token.as_str());
}
