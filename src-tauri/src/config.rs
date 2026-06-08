use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub api_key: Option<String>,
    #[serde(default = "default_smtp_host")]
    pub smtp_host: String,
    #[serde(default = "default_smtp_port")]
    pub smtp_port: u16,
    #[serde(default = "default_smtp_sender_email")]
    pub smtp_sender_email: String,
    #[serde(default = "default_smtp_username")]
    pub smtp_username: String,
    #[serde(default = "default_smtp_password")]
    pub smtp_password: String,
    #[serde(default = "default_litellm_host")]
    pub litellm_host: String,
    #[serde(default = "default_theme")]
    pub theme: String,
}

fn default_smtp_host() -> String {
    String::new()
}
fn default_smtp_port() -> u16 {
    465
}
fn default_smtp_sender_email() -> String {
    String::new()
}
fn default_smtp_username() -> String {
    String::new()
}
fn default_smtp_password() -> String {
    String::new()
}
fn default_litellm_host() -> String {
    String::new()
}
fn default_theme() -> String {
    "auto".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_key: None,
            smtp_host: default_smtp_host(),
            smtp_port: default_smtp_port(),
            smtp_sender_email: default_smtp_sender_email(),
            smtp_username: default_smtp_username(),
            smtp_password: default_smtp_password(),
            litellm_host: default_litellm_host(),
            theme: default_theme(),
        }
    }
}

fn config_path() -> PathBuf {
    let base = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("com.cybersh1t.tauri-app");
    fs::create_dir_all(&base).ok();
    base.join("config.json")
}

pub fn load_config() -> Config {
    let path = config_path();
    if path.exists() {
        match fs::read_to_string(&path) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
            Err(_) => Config::default(),
        }
    } else {
        Config::default()
    }
}

pub fn save_config(config: &Config) -> Result<(), String> {
    let path = config_path();
    let content = serde_json::to_string_pretty(config).map_err(|e| e.to_string())?;
    fs::write(&path, content).map_err(|e| e.to_string())?;
    Ok(())
}
