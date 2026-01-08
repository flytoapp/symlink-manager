use crate::commands::config::AppState;
use crate::models::{Item, ItemStatus};
use std::collections::HashMap;
use std::path::Path;
use tauri::State;

#[tauri::command]
pub fn list_items(source_path: String) -> Result<Vec<Item>, String> {
    let path = Path::new(&source_path);

    if !path.exists() {
        return Err(format!("Path does not exist: {}", source_path));
    }

    if !path.is_dir() {
        return Err(format!("Path is not a directory: {}", source_path));
    }

    let mut items = Vec::new();

    let entries =
        std::fs::read_dir(path).map_err(|e| format!("Failed to read directory: {}", e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
        let metadata = entry
            .metadata()
            .map_err(|e| format!("Failed to get metadata: {}", e))?;

        let name = entry.file_name().to_string_lossy().to_string();

        // Skip hidden files/folders
        if name.starts_with('.') {
            continue;
        }

        items.push(Item {
            name,
            is_directory: metadata.is_dir(),
            source_path: entry.path().to_string_lossy().to_string(),
            status: ItemStatus::Inactive,
            enabled: false,
            conflict_source: None,
        });
    }

    // Sort: directories first, then alphabetically
    items.sort_by(|a, b| {
        match (a.is_directory, b.is_directory) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        }
    });

    Ok(items)
}

#[tauri::command]
pub fn get_items_with_status(
    state: State<'_, AppState>,
    profile_id: String,
    source_id: String,
) -> Result<Vec<Item>, String> {
    let config = state.config.lock().map_err(|e| e.to_string())?;

    let profile = config
        .profiles
        .iter()
        .find(|p| p.id == profile_id)
        .ok_or("Profile not found")?;

    let source = profile
        .sources
        .iter()
        .find(|s| s.id == source_id)
        .ok_or("Source not found")?;

    let target_path = source.get_target_path(&profile.base_path);

    // Get all items from source directory
    let mut items = list_items(source.source_path.clone())?;

    // Build map of what other sources have enabled in the same target
    let mut other_enabled: HashMap<String, String> = HashMap::new();

    for other_source in &profile.sources {
        if other_source.id == source_id {
            continue;
        }
        let other_target = other_source.get_target_path(&profile.base_path);
        if other_target == target_path {
            for item in &other_source.enabled_items {
                other_enabled.insert(item.clone(), other_source.name.clone());
            }
        }
    }

    // Determine status for each item
    for item in &mut items {
        item.enabled = source.enabled_items.contains(&item.name);

        let symlink_path = Path::new(target_path).join(&item.name);

        if let Some(conflict_source) = other_enabled.get(&item.name) {
            item.status = ItemStatus::Conflict;
            item.conflict_source = Some(conflict_source.clone());
        } else if symlink_path.is_symlink() {
            // Check if symlink points to our source
            match std::fs::read_link(&symlink_path) {
                Ok(link_target) => {
                    let expected = Path::new(&source.source_path).join(&item.name);
                    // Normalize paths for comparison
                    let link_target_canonical = std::fs::canonicalize(&link_target).ok();
                    let expected_canonical = std::fs::canonicalize(&expected).ok();

                    match (link_target_canonical, expected_canonical) {
                        (Some(link), Some(exp)) if link == exp => {
                            item.status = ItemStatus::Active;
                        }
                        (None, _) => {
                            // Symlink target doesn't exist
                            item.status = ItemStatus::Broken;
                        }
                        _ => {
                            // Points to different location - could be from another source
                            // or manually created
                            if !item.enabled {
                                item.status = ItemStatus::Conflict;
                                item.conflict_source = Some("External".to_string());
                            } else {
                                item.status = ItemStatus::Broken;
                            }
                        }
                    }
                }
                Err(_) => {
                    item.status = ItemStatus::Broken;
                }
            }
        } else if symlink_path.exists() {
            // Regular file/folder exists with same name (not a symlink)
            item.status = ItemStatus::Conflict;
            item.conflict_source = Some("Existing file/folder".to_string());
        } else {
            item.status = ItemStatus::Inactive;
        }
    }

    Ok(items)
}

#[tauri::command]
pub fn validate_path(path: String) -> bool {
    Path::new(&path).exists()
}
