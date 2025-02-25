use crate::types::{TogglEntryId, TogglProjectId, TogglWorkspaceId};
use crate::{config::AppConfig, types::TogglEntryDescription};
use chrono::{DateTime, Utc};
use reqwest::header::CONTENT_TYPE;
use reqwest::{self, blocking::Client, Url};
use serde::{Deserialize, Serialize};
use serde_jsonrc;

pub fn start(
    config: &AppConfig,
    workspace_id: TogglWorkspaceId,
    project_id: TogglProjectId,
    description: TogglEntryDescription,
    start: DateTime<Utc>,
) -> Result<TimeEntry, TogglError> {
    let client = get_toggl_client(config.api_token.to_string());
    let running_entry = client.start_entry(&workspace_id, &project_id, description, start)?;
    return Ok(running_entry);
}

pub fn get_current_time_entry(config: &AppConfig) -> Option<TimeEntry> {
    let client = get_toggl_client(config.api_token.to_string());
    let entry = client
        .get_current_time_entry()
        .expect("failed to get current time entry");

    return entry;
}

pub fn stop(config: &AppConfig) -> Result<TimeEntry, TogglError> {
    let client = get_toggl_client(config.api_token.to_string());
    let running_entry = match client.get_current_time_entry() {
        Ok(Some(value)) => value,
        Ok(None) => return Err(TogglError::NoRuningTimeEntryFound),
        Err(error) => return Err(error),
    };

    let stopped_entry = client.stop_entry(&running_entry)?;

    return Ok(stopped_entry);
}

/// Swap the ongoing time entry with another one given a description and a project.
///
/// # Errors
///
/// This method fails if there is no ongoing time entry.
pub fn swap(
    config: &AppConfig,
    project_id: TogglProjectId,
    description: TogglEntryDescription,
) -> Result<(), TogglError> {
    let client = get_toggl_client(config.api_token.to_string());
    let running = match client.get_current_time_entry() {
        Ok(Some(value)) => value,
        Ok(None) => return Err(TogglError::NoRuningTimeEntryFound),
        Err(error) => return Err(error),
    };

    let desired = &TimeEntry {
        id: running.id,
        project_id,
        workspace_id: running.workspace_id,
        description,
        start: running.start,
        stop: running.stop,
        duration: running.duration,
    };

    if &running == desired {
        println!("No need to swap, desired entry is already running");
        return Ok(());
    }

    client.put_entry(desired)
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

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct TimeEntry {
    pub id: TogglEntryId,
    pub project_id: TogglProjectId,
    pub workspace_id: TogglWorkspaceId,
    pub description: TogglEntryDescription,
    pub start: DateTime<Utc>,
    pub stop: Option<DateTime<Utc>>,
    pub duration: i64,
}

#[derive(Debug)]
pub enum TogglError {
    Reqwest(reqwest::Error),
    SerdeJsonrc(serde_jsonrc::Error),
    NoRuningTimeEntryFound,
}

impl std::fmt::Display for TogglError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return match self {
            TogglError::Reqwest(e) => write!(f, "Call to Toggl API failed: {}", e),
            TogglError::SerdeJsonrc(e) => {
                write!(f, "Deserialization of Toggl API response failed: {}", e)
            }
            TogglError::NoRuningTimeEntryFound => write!(f, "No running time entry found"),
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

    fn post(&self, path: &str, body: &str) -> Result<String, reqwest::Error> {
        let url = self.base_url.join(path).expect("failed to build URL");

        let response = self
            .client
            .post(url)
            .body(body.to_string())
            .basic_auth(&(self.token), Some("api_token"))
            .header(CONTENT_TYPE, "application/json")
            .send()?;

        let body = response.text()?;

        return Ok(body);
    }

    fn patch(&self, path: &str) -> Result<String, reqwest::Error> {
        let url = self.base_url.join(path).expect("failed to build URL");

        let response = self
            .client
            .patch(url)
            .basic_auth(&(self.token), Some("api_token"))
            .send()?;
        // println!("Status: {}", response.status());
        let body = response.text()?;
        return Ok(body);
    }

    fn put(&self, path: &str, body: &str) -> Result<String, reqwest::Error> {
        let url = self.base_url.join(path).expect("failed to build URL");

        let response = self
            .client
            .put(url)
            .body(body.to_string())
            .basic_auth(&(self.token), Some("api_token"))
            .header(CONTENT_TYPE, "application/json")
            .send()?;

        let body = response.text()?;
        return Ok(body);
    }

    pub fn start_entry(
        &self,
        workspace_id: &TogglWorkspaceId,
        project_id: &TogglProjectId,
        description: TogglEntryDescription,
        start: DateTime<Utc>,
    ) -> Result<TimeEntry, TogglError> {
        // https://engineering.toggl.com/docs/api/time_entries#post-timeentries
        let request_body = serde_jsonrc::json!({
            "billable": false,
            "created_with": "t-rs",
            "description": &description,
            "duration": -1,  // must be a negative number for a running entry
            "project_id": &project_id,
            "start": start.to_rfc3339(),
            "workspace_id": &workspace_id.clone(),

        })
        .to_string();

        let path = format!(
            "/api/v9/workspaces/{workspace_id}/time_entries",
            workspace_id = workspace_id,
        );
        let response_body = self.post(&path, &request_body)?;

        let response: TimeEntry = serde_jsonrc::from_str(&response_body)?;
        return Ok(response);
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

    pub fn stop_entry(&self, entry: &TimeEntry) -> Result<TimeEntry, TogglError> {
        /*
        curl -u <email>:<password> \
        -X PATCH https://api.track.toggl.com/api/v9/workspaces/{workspace_id}/time_entries/{time_entry_id}/stop
         */
        let path = format!(
            "/api/v9/workspaces/{workspace_id}/time_entries/{time_entry_id}/stop",
            workspace_id = entry.workspace_id,
            time_entry_id = entry.id
        );
        let body = self.patch(&path)?;
        let stopped_entry: TimeEntry = serde_jsonrc::from_str(&body)?;
        return Ok(stopped_entry);
    }

    pub fn put_entry(&self, entry: &TimeEntry) -> Result<(), TogglError> {
        let path = format!(
            "https://api.track.toggl.com/api/v9/workspaces/{workspace_id}/time_entries/{time_entry_id}",
            workspace_id=entry.workspace_id,
            time_entry_id=entry.id,
        );

        let request_body = serde_jsonrc::json!({
            "created_with": "t-rs",
            "start": entry.start.to_rfc3339(),
            "description": &entry.description,
            "project-id": &entry.project_id,
            "tags":[],
            "billable": false,
            "workspace_id": entry.workspace_id,
            "duration": entry.duration,
        })
        .to_string();

        self.put(&path, &request_body)?;
        return Ok(());
    }
}
