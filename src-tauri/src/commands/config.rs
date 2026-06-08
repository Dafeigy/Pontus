use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::{AppHandle, Runtime};
use tauri_plugin_store::StoreExt;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    pub api_key: String,
    pub litellm_host: String,
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_sender_email: String,
    pub smtp_username: String,
    pub smtp_password: String,
    pub theme: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            litellm_host: "http://localhost:4000".to_string(),
            smtp_host: String::new(),
            smtp_port: 587,
            smtp_sender_email: String::new(),
            smtp_username: String::new(),
            smtp_password: String::new(),
            theme: "light".to_string(),
        }
    }
}

const STORE_NAME: &str = "config.json";
const CONFIG_KEY: &str = "app_config";

fn get_store<R: Runtime>(app: &AppHandle<R>) -> Result<Arc<tauri_plugin_store::Store<R>>, String> {
    app.store(STORE_NAME)
        .map_err(|e| format!("Failed to open store: {}", e))
}

#[tauri::command]
pub fn is_initialized_cmd(app: AppHandle) -> Result<bool, String> {
    let store = get_store(&app)?;
    if let Some(config) = store.get(CONFIG_KEY) {
        let config: AppConfig =
            serde_json::from_value(config.clone()).unwrap_or_default();
        Ok(!config.api_key.is_empty())
    } else {
        Ok(false)
    }
}

#[tauri::command]
pub fn get_config_cmd(app: AppHandle) -> Result<AppConfig, String> {
    let store = get_store(&app)?;
    if let Some(config) = store.get(CONFIG_KEY) {
        let config: AppConfig =
            serde_json::from_value(config.clone()).unwrap_or_default();
        Ok(config)
    } else {
        Ok(AppConfig::default())
    }
}

#[tauri::command]
pub fn save_config_cmd(app: AppHandle, config: AppConfig) -> Result<(), String> {
    let store = get_store(&app)?;
    let value = serde_json::to_value(&config).map_err(|e| format!("Serialize error: {}", e))?;
    store.set(CONFIG_KEY, value);
    store.save().map_err(|e| format!("Save error: {}", e))?;
    Ok(())
}

#[tauri::command]
pub fn reset_api_key_cmd(app: AppHandle) -> Result<AppConfig, String> {
    let store = get_store(&app)?;
    let mut config = if let Some(existing) = store.get(CONFIG_KEY) {
        serde_json::from_value::<AppConfig>(existing.clone()).unwrap_or_default()
    } else {
        AppConfig::default()
    };
    config.api_key = String::new();
    let value = serde_json::to_value(&config).map_err(|e| format!("Serialize error: {}", e))?;
    store.set(CONFIG_KEY, value);
    store.save().map_err(|e| format!("Save error: {}", e))?;
    Ok(config)
}
