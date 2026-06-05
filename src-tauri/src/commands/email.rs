use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::client::{Tls, TlsParameters};
use lettre::{Message, SmtpTransport, Transport};
use serde::{Deserialize, Serialize};

use super::config::{get_config_cmd, AppConfig};
use super::email_template::build_invite_email;
use super::litellm::{create_user_internal, generate_invitation_internal};
use tauri::AppHandle;

#[derive(Debug, Serialize, Deserialize)]
pub struct InviteResult {
    pub user_id: String,
    pub api_key: String,
    pub invitation_id: String,
    pub invitation_link: String,
    pub user_email: String,
}

fn send_email(
    config: &AppConfig,
    to_email: &str,
    subject: &str,
    html_body: String,
) -> Result<(), String> {
    let from_addr = format!("Litellm Admin <{}>", config.smtp_sender_email);
    let email = Message::builder()
        .from(
            from_addr
                .parse()
                .map_err(|e| format!("Invalid sender: {}", e))?,
        )
        .to(format!("<{}>", to_email)
            .parse()
            .map_err(|e| format!("Invalid recipient: {}", e))?)
        .subject(subject)
        .header(ContentType::TEXT_HTML)
        .body(html_body)
        .map_err(|e| format!("Email build error: {}", e))?;

    let creds = Credentials::new(
        config.smtp_username.clone(),
        config.smtp_password.clone(),
    );

    // Select TLS mode based on port: 465 → implicit TLS (SSL), others → STARTTLS
    let result = if config.smtp_port == 465 {
        let tls_params = TlsParameters::builder(config.smtp_host.clone())
            .dangerous_accept_invalid_certs(true)
            .build()
            .map_err(|e| format!("TLS params error: {}", e))?;
        SmtpTransport::relay(&config.smtp_host)
            .map_err(|e| format!("SMTP relay error: {}", e))?
            .tls(Tls::Wrapper(tls_params))
            .credentials(creds)
            .port(config.smtp_port)
            .build()
            .send(&email)
    } else {
        SmtpTransport::starttls_relay(&config.smtp_host)
            .map_err(|e| format!("SMTP STARTTLS relay error: {}", e))?
            .credentials(creds)
            .port(config.smtp_port)
            .build()
            .send(&email)
    };

    result.map(|_| ()).map_err(|e| format!("Email send error: {}", e))
}

#[tauri::command]
pub async fn send_invite_email(
    app: AppHandle,
    to_email: String,
    username: String,
    invitation_link: String,
    api_key: String,
) -> Result<(), String> {
    let config: AppConfig = get_config_cmd(app)?;
    validate_smtp_config(&config)?;
    let html = build_invite_email(&username, &to_email, &invitation_link, &api_key);
    send_email(&config, &to_email, "欢迎加入 Litellm — 您的账号已就绪", html)
}

fn validate_smtp_config(config: &AppConfig) -> Result<(), String> {
    if config.smtp_host.is_empty() {
        return Err("SMTP 服务器地址未配置，请先在设置页面完成 SMTP 配置".to_string());
    }
    if config.smtp_sender_email.is_empty() {
        return Err("发送者邮箱未配置，请先在设置页面完成 SMTP 配置".to_string());
    }
    if config.smtp_username.is_empty() {
        return Err("SMTP 用户名未配置，请先在设置页面完成 SMTP 配置".to_string());
    }
    if config.smtp_password.is_empty() {
        return Err("SMTP 密码未配置，请先在设置页面完成 SMTP 配置".to_string());
    }
    Ok(())
}

#[tauri::command]
pub async fn complete_invitation(
    app: AppHandle,
    user_id: String,
    user_email: String,
    user_alias: String,
    api_key: String,
) -> Result<serde_json::Value, String> {
    let result: Result<InviteResult, String> = tokio::task::spawn_blocking(move || {
        // Step 1: Generate invitation
        let invitation = generate_invitation_internal(&app, &user_id)?;

        // Build invitation link
        let config: AppConfig = get_config_cmd(app.clone())?;
        let base_url = config.litellm_host.trim_end_matches('/');
        let invitation_link = format!("{}/ui?invitation_id={}", base_url, invitation.id);

        // Step 2: Send email
        let html = build_invite_email(&user_alias, &user_email, &invitation_link, &api_key);
        send_email(&config, &user_email, "欢迎加入 Litellm — 您的账号已就绪", html)?;

        Ok(InviteResult {
            user_id,
            api_key,
            invitation_id: invitation.id,
            invitation_link,
            user_email,
        })
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?;

    let result = result?;
    serde_json::to_value(&result).map_err(|e| format!("Serialize error: {}", e))
}

#[tauri::command]
pub async fn invite_user(
    app: AppHandle,
    user_email: String,
    user_alias: String,
    user_role: String,
    key_alias: String,
) -> Result<serde_json::Value, String> {
    let result: Result<InviteResult, String> = tokio::task::spawn_blocking(move || {
        // Step 1: Create user
        let user = create_user_internal(&app, &user_email, &user_alias, &user_role, &key_alias)?;

        // Step 2: Generate invitation
        let invitation = generate_invitation_internal(&app, &user.user_id)?;

        // Build invitation link
        let config: AppConfig = get_config_cmd(app.clone())?;
        let base_url = config.litellm_host.trim_end_matches('/');
        let invitation_link = format!("{}/ui?invitation_id={}", base_url, invitation.id);

        // Step 3: Send email
        let html = build_invite_email(&user_alias, &user_email, &invitation_link, &user.key);
        send_email(&config, &user_email, "欢迎加入 Litellm — 您的账号已就绪", html)?;

        Ok(InviteResult {
            user_id: user.user_id,
            api_key: user.key,
            invitation_id: invitation.id,
            invitation_link,
            user_email,
        })
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?;

    let result = result?;
    serde_json::to_value(&result).map_err(|e| format!("Serialize error: {}", e))
}
