use crate::types::TogglProjectId;
use crate::{config::AppConfig, types::TogglEntryDescription};
use chrono::{DateTime, Utc};
use reqwest::{self, blocking::Client, Url};
use serde::{Deserialize, Serialize};

pub fn start(config: &AppConfig, project_id: TogglProjectId) {
    println!("Starting a time entry in {} project ...", project_id);
    println!("Timer started! token {}", config.api_token.as_str());
}

pub fn get_current_time_entry(config: &AppConfig) -> Option<TimeEntry> {
    let client = get_toggl_client(config.api_token.to_string());
    let entry = client
        .get_current_time_entry()
        .expect("failed to get current time entry");

    return entry;
}

pub fn stop(config: &AppConfig) {
    println!("Timer stopped! token {}", config.api_token.as_str());
}

struct TogglHttpClient {
    base_url: Url,
    token: String,
    client: Client,
}

fn get_toggl_client(token: String) -> TogglHttpClient {
    // Note: make sure the base URL has a `/` at the end
    let base_url =
        Url::parse("https://api.track.toggl.com/api/v9/").expect("failed to parse base URL");
    // println!("Base URL: {}", base_url);

    let client = reqwest::blocking::Client::new();
    let toggl = TogglHttpClient {
        base_url,
        token,
        client,
    };

    return toggl;
}

#[derive(Serialize, Deserialize)]
pub struct TimeEntry {
    pub project_id: TogglProjectId,
    pub description: TogglEntryDescription,
    pub start: DateTime<Utc>,
    // TODO: add start - at domain level calculate elapsed time since start
}

#[derive(Debug)]
enum TogglError {
    Reqwest(reqwest::Error),
    SerdeJsonrc(serde_jsonrc::Error),
}

impl std::fmt::Display for TogglError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return match self {
            TogglError::Reqwest(e) => write!(f, "Call to Toggl API failed: {}", e),
            TogglError::SerdeJsonrc(e) => {
                write!(f, "Deserialization of Toggl API response failed: {}", e)
            }
        };
    }
}

impl From<reqwest::Error> for TogglError {
    fn from(error: reqwest::Error) -> TogglError {
        return TogglError::Reqwest(error);
    }
}

impl From<serde_jsonrc::Error> for TogglError {
    fn from(error: serde_jsonrc::Error) -> TogglError {
        return TogglError::SerdeJsonrc(error);
    }
}

impl TogglHttpClient {
    fn get(&self, path: &str) -> Result<String, reqwest::Error> {
        let url = self.base_url.join(path).expect("failed to build URL");
        // println!("URL: {}", url);

        let response = self
            .client
            .get(url)
            .basic_auth(&(self.token), Some("api_token"))
            .send()?;
        // println!("Status: {}", response.status());
        let body = response.text()?;
        return Ok(body);
    }

    pub fn get_current_time_entry(&self) -> Result<Option<TimeEntry>, TogglError> {
        /*
        {
            body: {
                "id":2717980333,
                "workspace_id":1819588,
                "project_id":28787086,
                "task_id":null,
                "billable":false,
                "start":"2022-11-06T18:03:48+00:00",
                "stop":null,
                "duration":-1667757828,
                "description":"Coding",
                "tags":null,
                "tag_ids":null,
                "duronly":false,
                "at":"2022-11-06T18:03:48+00:00",
                "server_deleted_at":null,
                "user_id":2626092,
                "uid":2626092,
                "wid":1819588,
                "pid":28787086
            }
        }
         */
        let body = self.get("me/time_entries/current")?;
        // println!("{}", body);

        if body == "null" {
            return Ok(None);
        }

        let entry: TimeEntry = serde_jsonrc::from_str(&body)?;

        return Ok(Some(entry));
    }
}
