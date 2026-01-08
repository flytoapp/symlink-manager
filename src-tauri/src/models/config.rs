use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    pub version: u32,
    pub profiles: Vec<Profile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_profile_id: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            version: 1,
            profiles: Vec::new(),
            active_profile_id: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub base_path: String,
    pub sources: Vec<Source>,
}

impl Profile {
    pub fn new(name: String, base_path: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            base_path,
            sources: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    pub id: String,
    pub name: String,
    pub source_path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_path: Option<String>,
    #[serde(default)]
    pub enabled_items: Vec<String>,
}

impl Source {
    pub fn new(name: String, source_path: String, target_path: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            source_path,
            target_path,
            enabled_items: Vec::new(),
        }
    }

    /// Get the effective target path (own or profile default)
    pub fn get_target_path<'a>(&'a self, profile_base_path: &'a str) -> &'a str {
        self.target_path.as_deref().unwrap_or(profile_base_path)
    }
}
