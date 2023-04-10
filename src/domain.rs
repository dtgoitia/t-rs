use chrono::Duration;
use dialoguer::Select;
use dialoguer::{console::Term, theme::ColorfulTheme, FuzzySelect};
use regex::Regex;
use text_io;

use crate::config::AppConfig;
use crate::toggl::{self, TimeEntry};
use crate::types::{TogglEntryName, TogglProjectId, TogglProjectName};

pub fn show_toggl_status(config: &AppConfig) -> () {
    let entry = match toggl::get_current_time_entry(&config) {
        Some(value) => value,
        None => return println!("No time entry running"),
    };

    print_running_entry(config, &entry)
}

pub fn start_toggl_timer(config: &AppConfig) -> () {
    let selected = match select_entry(&config) {
        Some(selected) => selected,
        None => return println!("You apparently selected nothing :S"),
    };

    let now = chrono::Utc::now();

    match toggl::start(
        &config,
        selected.project_id,
        selected.description.to_string(),
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

pub fn swap_current_toggl_timer(config: &AppConfig) -> () {
    let entry = match toggl::get_current_time_entry(&config) {
        Some(value) => value,
        None => return println!("No time entry running"),
    };

    print_running_entry(config, &entry);

    let selected = match select_entry(&config) {
        Some(selected) => selected,
        None => return println!("You apparently selected nothing :S"),
    };

    match toggl::swap_description_or_project(&config, selected.project_id, selected.description) {
        Ok(_) => return,
        Err(error) => return println!("Failed to update Toggl time entry, reason: {}", error),
    };
}

pub fn shift_current_toggl_timer(config: &AppConfig) -> () {
    // get last 6h entries
    /*

    1.  show list of entries today and ask user to pick which one
        should modify:

        ```
        General   YLD   09:30 - 10:00  30m
        Meeting   YLD   10:00 -  ...   32m 1s

        Select current entry:  (usual autocompletion)
        ```

    2.  ask when should the new entry started, and clarify that if
        last meeting will be pushed back to accommodate this start
        time

        ```
        Select start time:
        ```
        find a good way of introducing either a time, or a duration
        (x mins ago)

    3.  Show confirmation screen to ensure that the user is aware
        of how will the previous entry and the new one look like:

        ```
        General   YLD   09:30 - 10:00  30m
        Meeting   YLD   10:00 - 10:15  15m
        Coding    YLD   10:15 -  ...   17m 4s

        Happy? [Y]/n
        ```

        And if the previous entry is being completely removed,
        raise an error and ask the user to use the toggl webapp
        directly to handle this situation.

    */
    let last_entries = toggl::get_last_entries(&config);

    println!("Which entry do you want to shift backwards?");
    let formated = last_entries
        .iter()
        .map(|entry| format_entry(entry, &config))
        .collect::<Vec<String>>();

    let to_shift_index = Select::with_theme(&ColorfulTheme::default())
        .items(&formated)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .unwrap()
        .expect("baaaad");

    let to_shift = &last_entries[to_shift_index];
    println!("{:?}", &to_shift);

    // Select start time
    println!("Type start time (hh:mm)");
    let line: String = text_io::read!("{}\n");
    let new_start = parse_time(line);
    println!("{:?}", new_start);

    println!("{:?}", to_shift);
    let shifted = to_shift.set_start(new_start.into());
    println!("{:?}", shifted);
    match toggl::replace_entry(config, shifted) {
        Ok(_) => return,
        Err(error) => return println!("Failed to update Toggl time entry, reason: {}", error),
    };
}

fn parse_time(raw: String) -> chrono::DateTime<chrono::Local> {
    let re = Regex::new(r"^(?P<h>\d{2}):?(?P<m>\d{2})$").unwrap();
    let captures = re.captures(&raw).unwrap();
    let h = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
    let m = captures.get(2).unwrap().as_str().parse::<u32>().unwrap();
    chrono::Local::today().and_hms(h, m, 0)
}

fn format_entry(entry: &TimeEntry, config: &AppConfig) -> String {
    let end = if entry.stop.is_none() {
        chrono::Utc::now()
    } else {
        entry.stop.unwrap()
    };

    let delta = format_duration(end.time() - entry.start.time());

    let project_name = get_project_name_by_id(entry.project_id, config);

    return format!(
        "{:<10}   {:<20}   {} - {}  {}",
        entry.description,
        project_name,
        entry.start.format("%H:%M:%S"),
        end.format("%H:%M:%S"),
        delta,
    );
}

fn print_running_entry(config: &AppConfig, entry: &TimeEntry) -> () {
    let now = chrono::Utc::now();
    let elapsed = format_duration(now - entry.start);
    let project_name = get_project_name_by_id(entry.project_id, &config);

    println!("{} @ {}   {}", entry.description, project_name, elapsed);
}

fn select_entry(config: &AppConfig) -> Option<SelectableTogglItem> {
    let (toggle_items, selection_items) = build_selection_items_from(&config);
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .items(&selection_items)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .expect("TODO: handle this error");

    return match selection {
        Some(index) => Some(toggle_items[index].clone()),
        None => return None,
    };
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

#[derive(Clone)]
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
