use crate::commands::litellm::types::*;
use crate::commands::litellm::{get_api_config, make_client};
use tauri::AppHandle;

// --- Internal helpers ---

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

// --- Tauri commands ---

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
