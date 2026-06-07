use std::io::{BufRead, BufReader};

use serde::{Deserialize, Serialize};

use crate::commands::config::get_config_cmd;
use tauri::AppHandle;
use tauri::Emitter;

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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct AccessGroup {
    pub access_group_id: String,
    pub access_group_name: String,
    pub description: Option<String>,
    pub access_model_names: Vec<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelTestResult {
    pub model: String,
    pub success: bool,
    pub latency_ms: u64,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct StreamChunk {
    pub stream_id: String,
    pub content: String,
    pub done: bool,
    pub error: Option<String>,
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

/// Internal: list access groups
pub fn list_access_groups_internal(app: &AppHandle) -> Result<Vec<AccessGroup>, String> {
    let (base_url, api_key) = get_api_config(app)?;

    let client = make_client();
    let response = client
        .get(format!("{}/v1/access_group", base_url))
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
    serde_json::from_str::<Vec<AccessGroup>>(&body)
        .map_err(|e| format!("Parse error: {}", e))
}

/// Internal: test a model's connectivity and latency
pub fn test_model_internal(app: &AppHandle, model: &str) -> Result<ModelTestResult, String> {
    let (base_url, api_key) = get_api_config(app)?;
    let start = std::time::Instant::now();

    let payload = serde_json::json!({
        "model": model,
        "messages": [
            {
                "role": "user",
                "content": " "
            }
        ],
        "temperature": 0.7,
        "max_tokens": 5
    });

    let client = make_client();
    let response = client
        .post(format!("{}/v1/chat/completions", base_url))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&payload)
        .send()
        .map_err(|e| format!("Request failed: {}", e))?;

    let latency_ms = start.elapsed().as_millis() as u64;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().unwrap_or_default();
        return Ok(ModelTestResult {
            model: model.to_string(),
            success: false,
            latency_ms,
            message: format!("HTTP {}: {}", status, body),
        });
    }

    let body = response
        .text()
        .map_err(|e| format!("Read body error: {}", e))?;

    let v: serde_json::Value =
        serde_json::from_str(&body).map_err(|e| format!("Parse error: {}", e))?;

    // Check if content or reasoning_content is non-empty
    let content = v["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("");
    let reasoning = v["choices"][0]["message"]["reasoning_content"]
        .as_str()
        .unwrap_or("");

    if content.is_empty() && reasoning.is_empty() {
        Ok(ModelTestResult {
            model: model.to_string(),
            success: false,
            latency_ms,
            message: "模型返回空内容".to_string(),
        })
    } else {
        Ok(ModelTestResult {
            model: model.to_string(),
            success: true,
            latency_ms,
            message: format!("{}ms", latency_ms),
        })
    }
}

/// Internal: stream chat completions via SSE, emitting events for each chunk
pub fn chat_stream_internal(
    app: &AppHandle,
    stream_id: String,
    model: String,
    messages: Vec<ChatMessage>,
) -> Result<(), String> {
    let (base_url, api_key) = get_api_config(app)?;

    let payload = serde_json::json!({
        "model": model,
        "messages": messages,
        "stream": true
    });

    let client = make_client();
    let response = client
        .post(format!("{}/v1/chat/completions", base_url))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&payload)
        .send()
        .map_err(|e| {
            let err_msg = format!("请求失败: {}", e);
            let _ = app.emit(
                "chat-stream-chunk",
                StreamChunk {
                    stream_id: stream_id.clone(),
                    content: String::new(),
                    done: true,
                    error: Some(err_msg.clone()),
                },
            );
            err_msg
        })?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().unwrap_or_default();
        let err_msg = format!("HTTP {}: {}", status, body);
        let _ = app.emit(
            "chat-stream-chunk",
            StreamChunk {
                stream_id: stream_id.clone(),
                content: String::new(),
                done: true,
                error: Some(err_msg.clone()),
            },
        );
        return Err(err_msg);
    }

    let reader = BufReader::new(response);

    for line in reader.lines() {
        let line = line.map_err(|e| {
            let err_msg = format!("读取流数据失败: {}", e);
            let _ = app.emit(
                "chat-stream-chunk",
                StreamChunk {
                    stream_id: stream_id.clone(),
                    content: String::new(),
                    done: true,
                    error: Some(err_msg.clone()),
                },
            );
            err_msg
        })?;

        if line.is_empty() {
            continue;
        }

        if !line.starts_with("data: ") {
            continue;
        }

        let data = &line[6..]; // Skip "data: "

        if data == "[DONE]" {
            let _ = app.emit(
                "chat-stream-chunk",
                StreamChunk {
                    stream_id: stream_id.clone(),
                    content: String::new(),
                    done: true,
                    error: None,
                },
            );
            return Ok(());
        }

        // Parse SSE data JSON, extract delta content
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(data) {
            let content = json["choices"][0]["delta"]["content"]
                .as_str()
                .unwrap_or("");
            let reasoning = json["choices"][0]["delta"]["reasoning_content"]
                .as_str()
                .unwrap_or("");

            let text = if !reasoning.is_empty() {
                reasoning
            } else {
                content
            };

            if !text.is_empty() {
                let _ = app.emit(
                    "chat-stream-chunk",
                    StreamChunk {
                        stream_id: stream_id.clone(),
                        content: text.to_string(),
                        done: false,
                        error: None,
                    },
                );
            }
        }
    }

    // Stream ended without [DONE] marker — treat as complete
    let _ = app.emit(
        "chat-stream-chunk",
        StreamChunk {
            stream_id: stream_id.clone(),
            content: String::new(),
            done: true,
            error: None,
        },
    );

    Ok(())
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

#[tauri::command]
pub async fn list_access_groups(
    app: AppHandle,
) -> Result<serde_json::Value, String> {
    let result = tokio::task::spawn_blocking(move || {
        list_access_groups_internal(&app)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))??;
    serde_json::to_value(&result).map_err(|e| format!("Serialize error: {}", e))
}

#[tauri::command]
pub async fn test_model(
    app: AppHandle,
    model: String,
) -> Result<serde_json::Value, String> {
    let result = tokio::task::spawn_blocking(move || {
        test_model_internal(&app, &model)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))??;
    serde_json::to_value(&result).map_err(|e| format!("Serialize error: {}", e))
}

#[tauri::command]
pub async fn chat_stream(
    app: AppHandle,
    stream_id: String,
    model: String,
    messages: Vec<ChatMessage>,
) -> Result<String, String> {
    let app_clone = app.clone();
    std::thread::spawn(move || {
        let _ = chat_stream_internal(&app_clone, stream_id, model, messages);
    });
    Ok("started".to_string())
}
