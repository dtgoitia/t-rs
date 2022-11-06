use dialoguer::{console::Term, theme::ColorfulTheme, FuzzySelect};

use crate::config::{AppConfig, TogglEntryName, TogglProjectId};

pub fn status(config: &AppConfig) {
    println!("Showing timer status! token {}", config.api_token.as_str());
}

pub fn start(config: &AppConfig) {
    println!("Timer started! token {}", config.api_token.as_str());
    let (toggle_items, selection_items) = build_selection_items_from(&config);
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .items(&selection_items)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .expect("TODO: handle this error");

    match selection {
        Some(index) => return toggle_start(&config, &(toggle_items[index])),
        None => println!("USer did not select anything"),
    };
}

pub fn stop(config: &AppConfig) {
    println!("Timer stopped! token {}", config.api_token.as_str());
}

struct TogglItem {
    project_id: TogglProjectId,
    entry_name: TogglEntryName,
}
type TogglItems = Vec<TogglItem>;
type SelectionItems = Vec<String>;

fn build_selection_items_from(config: &AppConfig) -> (TogglItems, SelectionItems) {
    let mut toggl_items: Vec<TogglItem> = Vec::new();
    let mut selection_items: SelectionItems = Vec::new();
    /*
    iterate over Project and its items
    build a vector of strings that represet the selectable items in the menu
    and using the vector index as a common ID:
    build a vector of struct that contain project ID, and one project item --> this will be used later to call toggl
     */

    for project in config.projects.iter() {
        for entry in project.entries.iter() {
            let toggl_item = TogglItem {
                project_id: project.id,
                entry_name: entry.to_string(),
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

fn toggle_start(config: &AppConfig, item: &TogglItem) {
    println!(
        "Starting {} in project {} ...",
        item.entry_name, item.project_id
    );
    println!("Timer started! token {}", config.api_token.as_str());
}
