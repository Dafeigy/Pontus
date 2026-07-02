use serde::{Deserialize, Serialize};

// --- User-related types ---

#[derive(Debug, Serialize)]
pub struct CreateUserRequest<'a> {
    pub user_email: &'a str,
    pub user_alias: &'a str,
    pub user_role: &'a str,
    pub key_alias: &'a str,
    pub models: &'a [String],
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
pub struct InvitationRequest<'a> {
    pub user_id: &'a str,
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

// --- Team types ---

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TeamInfo {
    pub team_alias: String,
    pub team_id: String,
    pub members: Vec<serde_json::Value>,
    pub keys: Vec<serde_json::Value>,
    pub spend: Option<f64>,
    pub tpm_limit: Option<i64>,
    pub rpm_limit: Option<i64>,
    pub max_budget: Option<f64>,
    pub models: Vec<String>,
    pub blocked: Option<bool>,
}

// --- Model / Chat types ---

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
    pub think_content: String,
    pub done: bool,
    pub error: Option<String>,
}
