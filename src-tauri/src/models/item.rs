use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ItemStatus {
    Active,
    Inactive,
    Broken,
    Conflict,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub name: String,
    pub is_directory: bool,
    pub source_path: String,
    pub status: ItemStatus,
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionStatus {
    pub can_create_symlinks: bool,
    pub requires_elevation: bool,
    pub is_developer_mode: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SymlinkResult {
    pub success: bool,
    pub item_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}
