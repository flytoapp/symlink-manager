use crate::models::{AppConfig, Profile, Source};
use crate::services::ConfigService;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::State;

pub struct AppState {
    pub config: Mutex<AppConfig>,
    pub config_path: PathBuf,
}

fn save_config_internal(state: &State<'_, AppState>) -> Result<(), String> {
    let config = state.config.lock().map_err(|e| e.to_string())?;
    ConfigService::save(&state.config_path, &config)
}

#[tauri::command]
pub fn load_config(state: State<'_, AppState>) -> Result<AppConfig, String> {
    let config = state.config.lock().map_err(|e| e.to_string())?;
    Ok(config.clone())
}

#[tauri::command]
pub fn save_config(state: State<'_, AppState>) -> Result<(), String> {
    save_config_internal(&state)
}

#[tauri::command]
pub fn create_profile(
    state: State<'_, AppState>,
    name: String,
    base_path: String,
) -> Result<Profile, String> {
    let profile = Profile::new(name, base_path);

    {
        let mut config = state.config.lock().map_err(|e| e.to_string())?;
        config.profiles.push(profile.clone());
    }

    save_config_internal(&state)?;
    Ok(profile)
}

#[tauri::command]
pub fn update_profile(state: State<'_, AppState>, profile: Profile) -> Result<Profile, String> {
    {
        let mut config = state.config.lock().map_err(|e| e.to_string())?;
        if let Some(existing) = config.profiles.iter_mut().find(|p| p.id == profile.id) {
            *existing = profile.clone();
        } else {
            return Err("Profile not found".to_string());
        }
    }

    save_config_internal(&state)?;
    Ok(profile)
}

#[tauri::command]
pub fn delete_profile(state: State<'_, AppState>, profile_id: String) -> Result<(), String> {
    // First, clean up symlinks for all sources in this profile
    {
        let config = state.config.lock().map_err(|e| e.to_string())?;
        if let Some(profile) = config.profiles.iter().find(|p| p.id == profile_id) {
            for source in &profile.sources {
                let target_path = source.get_target_path(&profile.base_path);
                for item_name in &source.enabled_items {
                    let symlink_path = std::path::Path::new(target_path).join(item_name);
                    if symlink_path.is_symlink() {
                        let _ = remove_symlink(&symlink_path);
                    }
                }
            }
        }
    }

    {
        let mut config = state.config.lock().map_err(|e| e.to_string())?;
        config.profiles.retain(|p| p.id != profile_id);
    }

    save_config_internal(&state)?;
    Ok(())
}

#[tauri::command]
pub fn create_source(
    state: State<'_, AppState>,
    profile_id: String,
    name: String,
    source_path: String,
    target_path: Option<String>,
) -> Result<Source, String> {
    let source = Source::new(name, source_path, target_path);

    {
        let mut config = state.config.lock().map_err(|e| e.to_string())?;
        let profile = config
            .profiles
            .iter_mut()
            .find(|p| p.id == profile_id)
            .ok_or("Profile not found")?;
        profile.sources.push(source.clone());
    }

    save_config_internal(&state)?;
    Ok(source)
}

#[tauri::command]
pub fn update_source(
    state: State<'_, AppState>,
    profile_id: String,
    source: Source,
) -> Result<Source, String> {
    {
        let mut config = state.config.lock().map_err(|e| e.to_string())?;
        let profile = config
            .profiles
            .iter_mut()
            .find(|p| p.id == profile_id)
            .ok_or("Profile not found")?;

        if let Some(existing) = profile.sources.iter_mut().find(|s| s.id == source.id) {
            *existing = source.clone();
        } else {
            return Err("Source not found".to_string());
        }
    }

    save_config_internal(&state)?;
    Ok(source)
}

#[tauri::command]
pub fn delete_source(
    state: State<'_, AppState>,
    profile_id: String,
    source_id: String,
) -> Result<(), String> {
    // First, clean up symlinks for this source
    {
        let config = state.config.lock().map_err(|e| e.to_string())?;
        if let Some(profile) = config.profiles.iter().find(|p| p.id == profile_id) {
            if let Some(source) = profile.sources.iter().find(|s| s.id == source_id) {
                let target_path = source.get_target_path(&profile.base_path);
                for item_name in &source.enabled_items {
                    let symlink_path = std::path::Path::new(target_path).join(item_name);
                    if symlink_path.is_symlink() {
                        let _ = remove_symlink(&symlink_path);
                    }
                }
            }
        }
    }

    {
        let mut config = state.config.lock().map_err(|e| e.to_string())?;
        let profile = config
            .profiles
            .iter_mut()
            .find(|p| p.id == profile_id)
            .ok_or("Profile not found")?;
        profile.sources.retain(|s| s.id != source_id);
    }

    save_config_internal(&state)?;
    Ok(())
}

#[tauri::command]
pub fn set_active_profile(
    state: State<'_, AppState>,
    profile_id: Option<String>,
) -> Result<(), String> {
    {
        let mut config = state.config.lock().map_err(|e| e.to_string())?;
        config.active_profile_id = profile_id;
    }

    save_config_internal(&state)?;
    Ok(())
}

// Helper function for cross-platform symlink removal
fn remove_symlink(path: &std::path::Path) -> std::io::Result<()> {
    #[cfg(windows)]
    {
        let metadata = std::fs::symlink_metadata(path)?;
        if metadata.is_dir() {
            std::fs::remove_dir(path)
        } else {
            std::fs::remove_file(path)
        }
    }

    #[cfg(unix)]
    {
        std::fs::remove_file(path)
    }
}
