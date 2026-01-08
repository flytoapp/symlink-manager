use crate::models::AppConfig;
use std::fs;
use std::path::Path;

pub struct ConfigService;

impl ConfigService {
    pub fn load(path: &Path) -> Result<AppConfig, String> {
        if !path.exists() {
            return Ok(AppConfig::default());
        }

        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read config file: {}", e))?;

        serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse config file: {}", e))
    }

    pub fn save(path: &Path, config: &AppConfig) -> Result<(), String> {
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create config directory: {}", e))?;
        }

        let content = serde_json::to_string_pretty(config)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;

        fs::write(path, content)
            .map_err(|e| format!("Failed to write config file: {}", e))
    }
}
