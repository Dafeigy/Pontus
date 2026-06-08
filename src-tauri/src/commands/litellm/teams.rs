use crate::commands::litellm::types::TeamInfo;
use crate::commands::litellm::{get_api_config, make_client};
use tauri::AppHandle;

// --- Internal helpers ---

/// Internal: list teams
pub fn list_teams_internal(app: &AppHandle) -> Result<Vec<TeamInfo>, String> {
    let (base_url, api_key) = get_api_config(app)?;

    let client = make_client();
    let response = client
        .get(format!("{}/team/list", base_url))
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
    serde_json::from_str::<Vec<TeamInfo>>(&body)
        .map_err(|e| format!("Parse error: {}", e))
}

// --- Tauri commands ---

#[tauri::command]
pub async fn list_teams(
    app: AppHandle,
) -> Result<serde_json::Value, String> {
    let result = tokio::task::spawn_blocking(move || {
        list_teams_internal(&app)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))??;
    serde_json::to_value(&result).map_err(|e| format!("Serialize error: {}", e))
}
