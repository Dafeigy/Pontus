use serde::{Deserialize, Serialize};

use crate::commands::config::get_config_cmd;
use tauri::AppHandle;

// --- Request / Response types for Litellm API ---

#[derive(Debug, Serialize)]
struct CreateUserRequest<'a> {
    user_email: &'a str,
    user_alias: &'a str,
    user_role: &'a str,
    key_alias: &'a str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateUserResponse {
    pub user_id: String,
    pub key: String,
    pub user_email: String,
    pub user_role: String,
    pub user_alias: Option<String>,
}

#[derive(Debug, Serialize)]
struct InvitationRequest<'a> {
    user_id: &'a str,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InvitationResponse {
    pub id: String,
    pub user_id: String,
    pub is_accepted: bool,
    pub expires_at: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserInfo {
    pub user_id: String,
    pub user_email: Option<String>,
    pub user_alias: Option<String>,
    pub user_role: Option<String>,
    pub spend: Option<f64>,
    pub key_count: Option<i32>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserListResponse {
    pub users: Vec<UserInfo>,
    pub total: i32,
    pub page: i32,
    pub page_size: i32,
    pub total_pages: i32,
}

// --- Internal API helpers (return typed structs) ---

fn make_client() -> reqwest::blocking::Client {
    reqwest::blocking::Client::new()
}

fn get_api_config(app: &AppHandle) -> Result<(String, String), String> {
    let config = get_config_cmd(app.clone())?;
    if config.api_key.is_empty() {
        return Err("API Key is not configured".to_string());
    }
    let base_url = config.litellm_host.trim_end_matches('/').to_string();
    Ok((base_url, config.api_key))
}

/// Internal: create user, returns typed struct
pub fn create_user_internal(
    app: &AppHandle,
    user_email: &str,
    user_alias: &str,
    user_role: &str,
    key_alias: &str,
) -> Result<CreateUserResponse, String> {
    let (base_url, api_key) = get_api_config(app)?;

    let payload = CreateUserRequest {
        user_email,
        user_alias,
        user_role,
        key_alias,
    };

    let client = make_client();
    let response = client
        .post(format!("{}/user/new", base_url))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&payload)
        .send()
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().unwrap_or_default();
        return Err(format!("API error {}: {}", status, body));
    }

    let body = response
        .text()
        .map_err(|e| format!("Read body error: {}", e))?;
    serde_json::from_str::<CreateUserResponse>(&body)
        .map_err(|e| format!("Parse error: {}", e))
}

/// Internal: generate invitation, returns typed struct
pub fn generate_invitation_internal(app: &AppHandle, user_id: &str) -> Result<InvitationResponse, String> {
    let (base_url, api_key) = get_api_config(app)?;

    let payload = InvitationRequest { user_id };

    let client = make_client();
    let response = client
        .post(format!("{}/invitation/new", base_url))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&payload)
        .send()
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().unwrap_or_default();
        return Err(format!("API error {}: {}", status, body));
    }

    let body = response
        .text()
        .map_err(|e| format!("Read body error: {}", e))?;
    serde_json::from_str::<InvitationResponse>(&body)
        .map_err(|e| format!("Parse error: {}", e))
}

/// Internal: list users, returns typed struct
pub fn list_users_internal(app: &AppHandle, page: u32, page_size: u32) -> Result<UserListResponse, String> {
    let (base_url, api_key) = get_api_config(app)?;

    let client = make_client();
    let response = client
        .get(format!(
            "{}/user/list?page={}&page_size={}",
            base_url, page, page_size
        ))
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().unwrap_or_default();
        return Err(format!("API error {}: {}", status, body));
    }

    let body = response
        .text()
        .map_err(|e| format!("Read body error: {}", e))?;
    serde_json::from_str::<UserListResponse>(&body)
        .map_err(|e| format!("Parse error: {}", e))
}

// --- Tauri commands (async wrappers around blocking I/O) ---

#[tauri::command]
pub async fn create_user(
    app: AppHandle,
    user_email: String,
    user_alias: String,
    user_role: String,
    key_alias: String,
) -> Result<serde_json::Value, String> {
    let result = tokio::task::spawn_blocking(move || {
        create_user_internal(&app, &user_email, &user_alias, &user_role, &key_alias)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))??;
    serde_json::to_value(&result).map_err(|e| format!("Serialize error: {}", e))
}

#[tauri::command]
pub async fn generate_invitation(
    app: AppHandle,
    user_id: String,
) -> Result<serde_json::Value, String> {
    let result = tokio::task::spawn_blocking(move || {
        generate_invitation_internal(&app, &user_id)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))??;
    serde_json::to_value(&result).map_err(|e| format!("Serialize error: {}", e))
}

#[tauri::command]
pub async fn list_users(
    app: AppHandle,
    page: u32,
    page_size: u32,
) -> Result<serde_json::Value, String> {
    let result = tokio::task::spawn_blocking(move || {
        list_users_internal(&app, page, page_size)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))??;
    serde_json::to_value(&result).map_err(|e| format!("Serialize error: {}", e))
}
