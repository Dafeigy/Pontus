pub mod types;
pub mod users;
pub mod models;
pub mod teams;

// Re-export Tauri commands so lib.rs imports don't change
pub use users::{create_user, generate_invitation, list_users, create_user_internal, generate_invitation_internal};
pub use models::{list_access_groups, test_model, chat_stream};
pub use teams::list_teams;

use crate::commands::config::get_config_cmd;
use tauri::AppHandle;

pub fn make_client() -> reqwest::blocking::Client {
    reqwest::blocking::Client::new()
}

pub fn get_api_config(app: &AppHandle) -> Result<(String, String), String> {
    let config = get_config_cmd(app.clone())?;
    if config.api_key.is_empty() {
        return Err("API Key is not configured".to_string());
    }
    let base_url = config.litellm_host.trim_end_matches('/').to_string();
    Ok((base_url, config.api_key))
}
