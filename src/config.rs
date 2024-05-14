use serde::{Deserialize, Serialize};
use std::{fs, path::Path, process};

use crate::types::{TogglEntryName, TogglProjectId, TogglProjectName, TogglWorkspaceId};

pub fn load(config_path: &Path, credentials_path: &Path) -> AppConfig {
    abort_if_config_file_does_not_exit(config_path);
    abort_if_credentials_file_does_not_exit(credentials_path);

    let config = parse(config_path, credentials_path);

    return config;
}

pub struct AppConfig {
    pub projects: Vec<Project>,
    pub api_token: TogglApiToken,
}

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub id: TogglProjectId,
    pub name: TogglProjectName,
    pub entries: Vec<TogglEntryName>,
    pub workspace_id: TogglWorkspaceId,
}

#[derive(Serialize, Deserialize)]
struct RawConfig {
    projects: Vec<Project>,
}

#[derive(Serialize, Deserialize)]
struct RawCredentials {
    toggle_api_token: TogglApiToken,
}

type TogglApiToken = String;

fn parse(config_path: &Path, credentials_path: &Path) -> AppConfig {
    let raw_config: RawConfig = parse_config(config_path);
    let raw_credentials: RawCredentials = parse_credentials(credentials_path);

    let config = AppConfig {
        projects: raw_config.projects,
        api_token: raw_credentials.toggle_api_token,
    };

    return config;
}

fn parse_config(config_path: &Path) -> RawConfig {
    let raw = fs::read_to_string(config_path)
        .expect("TODO: handle result better, failed to read config file");

    let raw_config: RawConfig =
        serde_jsonrc::from_str(&raw).expect("TODO: handle result better, failed to parse JSON");

    return raw_config;
}

fn parse_credentials(credentials_path: &Path) -> RawCredentials {
    let raw = fs::read_to_string(credentials_path)
        .expect("TODO: handle result better, failed to read config file");

    let raw_credentials: RawCredentials =
        serde_jsonrc::from_str(&raw).expect("TODO: handle result better, failed to parse JSON");

    return raw_credentials;
}

fn abort_if_config_file_does_not_exit(path: &Path) {
    let message = format!("Please create config file at: {}", path.display());
    abort_if_file_does_not_exit(path, message)
}

fn abort_if_credentials_file_does_not_exit(path: &Path) {
    let message = format!("Please create credentials file at: {}", path.display());
    abort_if_file_does_not_exit(path, message);
}

// TODO: use Result instead of panicking
fn abort_if_file_does_not_exit(path: &Path, message: String) {
    if path.exists() {
        return;
    }

    println!("{}", message);
    process::exit(1);
}
