use chrono::Duration;
use dialoguer::{console::Term, theme::ColorfulTheme, FuzzySelect};

use crate::config::AppConfig;
use crate::toggl;
use crate::types::{TogglEntryName, TogglProjectId, TogglProjectName};

pub fn show_toggl_status(config: &AppConfig) {
    let entry = match toggl::get_current_time_entry(&config) {
        Some(value) => value,
        None => return println!("No time entry running"),
    };

    let now = chrono::Utc::now();
    let elapsed = format_duration(now - entry.start);
    let project_name = get_project_name_by_id(entry.project_id, &config);

    println!("{} @ {}   {}", entry.description, project_name, elapsed);
}

pub fn start_toggl_timer(config: &AppConfig) {
    let (toggle_items, selection_items) = build_selection_items_from(&config);
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .items(&selection_items)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .expect("TODO: handle this error");

    let entry = match selection {
        Some(index) => &toggle_items[index],
        None => return println!("You apparently selected nothing :S"),
    };

    let now = chrono::Utc::now();

    match toggl::start(
        &config,
        entry.project_id,
        entry.description.to_string(),
        now,
    ) {
        Ok(_) => return println!("Successfully started"),
        Err(error) => return println!("Failed to start Toggl time entry, reason: {}", error),
    };
}

pub fn stop_toggl_timer(config: &AppConfig) -> () {
    let stopped_entry = match toggl::stop(&config) {
        Ok(value) => value,
        Err(error) => return println!("{:#?}", error),
    };

    let end = match stopped_entry.stop {
        Some(value) => value,
        None => {
            return println!(
                "Expected stopped entry to have an \"end\", but got this instead:\n{:#?}",
                stopped_entry
            )
        }
    };

    let elapsed = format_duration(end - stopped_entry.start);
    let project_name = get_project_name_by_id(stopped_entry.project_id, &config);
    println!(
        "Stopped: {description} @ {project}  {elapsed}",
        description = stopped_entry.description,
        project = project_name,
        elapsed = elapsed
    );
}

fn get_project_name_by_id(id: TogglProjectId, config: &AppConfig) -> TogglProjectName {
    for project in config.projects.iter() {
        if project.id == id {
            return project.name.to_string();
        }
    }

    // TODO: return this as an error and let the caller print message and handle it
    println!("WARNING: could not find project name for {} project ID", id);
    return id.to_string();
}

fn format_duration(elapsed: Duration) -> String {
    let days = elapsed.num_days();
    let h = elapsed.num_hours() % 24;
    let m = elapsed.num_minutes() % 60;
    let s = elapsed.num_seconds() % 60;

    let mut chunks: Vec<String> = vec![];
    if days > 0 {
        chunks.push(format!("{} days", days));
    }

    if h > 0 {
        chunks.push(format!("{}h", h));
    }

    if m > 0 {
        chunks.push(format!("{}m", m));
    }

    if s == 0 {
        chunks.push("0s".to_string());
    } else {
        chunks.push(format!("{}s", s));
    }

    return chunks.join(" ");
}

struct SelectableTogglItem {
    pub project_id: TogglProjectId,
    pub description: TogglEntryName,
}
type TogglItems = Vec<SelectableTogglItem>;
type SelectionItems = Vec<String>;

fn build_selection_items_from(config: &AppConfig) -> (TogglItems, SelectionItems) {
    let mut toggl_items: Vec<SelectableTogglItem> = Vec::new();
    let mut selection_items: SelectionItems = Vec::new();
    /*
    iterate over Project and its items
    build a vector of strings that represet the selectable items in the menu
    and using the vector index as a common ID:
    build a vector of struct that contain project ID, and one project item --> this will be used later to call toggl
     */

    for project in config.projects.iter() {
        for entry in project.entries.iter() {
            let toggl_item = SelectableTogglItem {
                project_id: project.id,
                description: entry.to_string(),
            };
            toggl_items.push(toggl_item);

            let formatted_selection_item =
                format_selection_item(project.name.to_string(), entry.to_string());
            selection_items.push(formatted_selection_item);
        }
    }

    return (toggl_items, selection_items);
}

fn format_selection_item(project_name: String, entry_name: String) -> String {
    return format!("{} @ {}", entry_name, project_name);
}
