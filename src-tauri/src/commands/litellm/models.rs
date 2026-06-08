use std::io::{BufRead, BufReader};

use crate::commands::litellm::types::*;
use crate::commands::litellm::{get_api_config, make_client};
use tauri::AppHandle;
use tauri::Emitter;

// --- Internal helpers ---

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

// --- Tauri commands ---

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
