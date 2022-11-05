use crate::config::AppConfig;

pub fn status(config: &AppConfig) {
    println!("Showing timer status! token {}", config.api_token.as_str());
}

pub fn start(config: &AppConfig) {
    println!("Timer started! token {}", config.api_token.as_str());
}

pub fn stop(config: &AppConfig) {
    println!("Timer stopped! token {}", config.api_token.as_str());
}
