use crate::commands::config::AppState;
use crate::models::{PermissionStatus, SymlinkResult};
use crate::services::ConfigService;
use std::path::Path;
use tauri::State;

#[cfg(unix)]
use std::os::unix::fs::symlink;

#[cfg(windows)]
use std::os::windows::fs::{symlink_dir, symlink_file};

#[tauri::command]
pub fn toggle_item(
    state: State<'_, AppState>,
    profile_id: String,
    source_id: String,
    item_name: String,
    enabled: bool,
) -> Result<SymlinkResult, String> {
    let (_source_path, target_path, item_source_path) = {
        let mut config = state.config.lock().map_err(|e| e.to_string())?;

        let profile = config
            .profiles
            .iter_mut()
            .find(|p| p.id == profile_id)
            .ok_or("Profile not found")?;

        let source = profile
            .sources
            .iter_mut()
            .find(|s| s.id == source_id)
            .ok_or("Source not found")?;

        let target = source.get_target_path(&profile.base_path).to_string();
        let src = source.source_path.clone();
        let item_src = Path::new(&src).join(&item_name).to_string_lossy().to_string();

        // Update enabled state
        if enabled {
            if !source.enabled_items.contains(&item_name) {
                source.enabled_items.push(item_name.clone());
            }
        } else {
            source.enabled_items.retain(|i| i != &item_name);
        }

        (src, target, item_src)
    };

    // Save config
    {
        let config = state.config.lock().map_err(|e| e.to_string())?;
        ConfigService::save(&state.config_path, &config)?;
    }

    // Handle the actual symlink
    let item_source = Path::new(&item_source_path);
    let symlink_path = Path::new(&target_path).join(&item_name);

    if enabled {
        // Create symlink
        if symlink_path.exists() || symlink_path.is_symlink() {
            // Rollback enabled state
            rollback_enabled_state(&state, &profile_id, &source_id, &item_name)?;
            return Ok(SymlinkResult {
                success: false,
                item_name,
                error: Some("Target location already has a file or folder with this name".to_string()),
            });
        }

        // Ensure target directory exists
        if let Some(parent) = symlink_path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create target directory: {}", e))?;
            }
        }

        match create_symlink(item_source, &symlink_path) {
            Ok(_) => Ok(SymlinkResult {
                success: true,
                item_name,
                error: None,
            }),
            Err(e) => {
                // Rollback enabled state
                rollback_enabled_state(&state, &profile_id, &source_id, &item_name)?;

                Ok(SymlinkResult {
                    success: false,
                    item_name,
                    error: Some(format_symlink_error(e)),
                })
            }
        }
    } else {
        // Remove symlink
        if symlink_path.is_symlink() {
            match remove_symlink(&symlink_path) {
                Ok(_) => Ok(SymlinkResult {
                    success: true,
                    item_name,
                    error: None,
                }),
                Err(e) => Ok(SymlinkResult {
                    success: false,
                    item_name,
                    error: Some(e.to_string()),
                }),
            }
        } else {
            Ok(SymlinkResult {
                success: true,
                item_name,
                error: None,
            })
        }
    }
}

fn rollback_enabled_state(
    state: &State<'_, AppState>,
    profile_id: &str,
    source_id: &str,
    item_name: &str,
) -> Result<(), String> {
    {
        let mut config = state.config.lock().map_err(|e| e.to_string())?;
        if let Some(profile) = config.profiles.iter_mut().find(|p| p.id == profile_id) {
            if let Some(source) = profile.sources.iter_mut().find(|s| s.id == source_id) {
                source.enabled_items.retain(|i| i != item_name);
            }
        }
    }
    let config = state.config.lock().map_err(|e| e.to_string())?;
    ConfigService::save(&state.config_path, &config)
}

#[cfg(unix)]
fn create_symlink(source: &Path, target: &Path) -> std::io::Result<()> {
    symlink(source, target)
}

#[cfg(windows)]
fn create_symlink(source: &Path, target: &Path) -> std::io::Result<()> {
    if source.is_dir() {
        symlink_dir(source, target)
    } else {
        symlink_file(source, target)
    }
}

fn remove_symlink(path: &Path) -> std::io::Result<()> {
    #[cfg(windows)]
    {
        // On Windows, directory symlinks must be removed with remove_dir,
        // and file symlinks with remove_file. Using the wrong one gives
        // "Access is denied" (error 5).
        //
        // Try remove_dir first (works for directory symlinks), then fall back
        // to remove_file (for file symlinks). This is more reliable than
        // checking metadata.is_dir() which can be inconsistent for symlinks.
        match std::fs::remove_dir(path) {
            Ok(()) => Ok(()),
            Err(e) if e.raw_os_error() == Some(145) => {
                // ERROR_DIR_NOT_EMPTY - shouldn't happen for symlinks, but just in case
                Err(e)
            }
            Err(_) => {
                // Try remove_file for file symlinks
                std::fs::remove_file(path)
            }
        }
    }

    #[cfg(unix)]
    {
        std::fs::remove_file(path)
    }
}

#[cfg(windows)]
fn format_symlink_error(e: std::io::Error) -> String {
    match e.raw_os_error() {
        Some(1314) => {
            "Permission denied: Creating symbolic links requires either:\n\
             1. Enable Developer Mode in Windows Settings (Settings > Update & Security > For developers)\n\
             2. Run this application as Administrator"
                .to_string()
        }
        _ => e.to_string(),
    }
}

#[cfg(unix)]
fn format_symlink_error(e: std::io::Error) -> String {
    e.to_string()
}

#[tauri::command]
pub fn check_symlink_permissions() -> PermissionStatus {
    #[cfg(unix)]
    {
        PermissionStatus {
            can_create_symlinks: true,
            requires_elevation: false,
            is_developer_mode: false,
            error_message: None,
        }
    }

    #[cfg(windows)]
    {
        // Test by trying to create a symlink in temp
        let temp_dir = std::env::temp_dir();
        let test_source = temp_dir.join("symlink_manager_test_source");
        let test_target = temp_dir.join("symlink_manager_test_link");

        // Create test source file
        let _ = std::fs::write(&test_source, "test");

        // Clean up any previous test
        let _ = std::fs::remove_file(&test_target);

        let result = symlink_file(&test_source, &test_target);

        // Cleanup
        let _ = std::fs::remove_file(&test_source);
        let _ = std::fs::remove_file(&test_target);

        match result {
            Ok(_) => PermissionStatus {
                can_create_symlinks: true,
                requires_elevation: false,
                is_developer_mode: check_developer_mode(),
                error_message: None,
            },
            Err(e) => {
                let is_privilege_error = e.raw_os_error() == Some(1314);
                PermissionStatus {
                    can_create_symlinks: false,
                    requires_elevation: is_privilege_error,
                    is_developer_mode: check_developer_mode(),
                    error_message: Some(format_symlink_error(e)),
                }
            }
        }
    }
}

#[cfg(windows)]
fn check_developer_mode() -> bool {
    use winreg::enums::*;
    use winreg::RegKey;

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    if let Ok(key) =
        hklm.open_subkey(r"SOFTWARE\Microsoft\Windows\CurrentVersion\AppModelUnlock")
    {
        if let Ok(value) = key.get_value::<u32, _>("AllowDevelopmentWithoutDevLicense") {
            return value == 1;
        }
    }
    false
}
