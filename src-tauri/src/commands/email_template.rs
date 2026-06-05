pub fn build_invite_email(
    username: &str,
    user_email: &str,
    invitation_link: &str,
    api_key: &str,
) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="zh-CN">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>欢迎使用 Litellm</title>
</head>
<body style="margin:0;padding:0;background-color:#f3f4f6;font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,'Helvetica Neue',Arial,sans-serif;">
<table width="100%" cellpadding="0" cellspacing="0" style="background-color:#f3f4f6;padding:24px 0;">
  <tr>
    <td align="center">
      <table width="100%" cellpadding="0" cellspacing="0" style="max-width:480px;background-color:#ffffff;border-radius:12px;overflow:hidden;box-shadow:0 2px 8px rgba(0,0,0,0.06);">
        <!-- Header -->
        <tr>
          <td style="background:linear-gradient(135deg,#2563eb,#1d4ed8);padding:32px 24px;text-align:center;">
            <h1 style="color:#ffffff;font-size:22px;margin:0;font-weight:700;">🚀 欢迎使用 Litellm</h1>
            <p style="color:#bfdbfe;font-size:14px;margin:8px 0 0 0;">您的模型服务已就绪</p>
          </td>
        </tr>
        <!-- Body -->
        <tr>
          <td style="padding:28px 24px 24px 24px;">
            <p style="color:#374151;font-size:15px;line-height:1.6;margin:0 0 20px 0;">
              您好，<strong style="color:#1d4ed8;">{username}</strong>：
            </p>
            <p style="color:#374151;font-size:15px;line-height:1.6;margin:0 0 24px 0;">
              您的 Litellm 账号已创建完成。以下是您的账号信息，请妥善保管：
            </p>

            <!-- Info Card -->
            <table width="100%" cellpadding="0" cellspacing="0" style="background-color:#f8fafc;border:1px solid #e2e8f0;border-radius:8px;padding:16px;margin-bottom:24px;">
              <tr>
                <td style="padding:4px 0;">
                  <span style="color:#64748b;font-size:13px;">注册邮箱/登录邮箱</span><br>
                  <span style="color:#1e293b;font-size:14px;font-weight:500;">{user_email}</span>
                </td>
              </tr>
              <tr>
                <td style="padding:12px 0 4px 0;">
                  <span style="color:#64748b;font-size:13px;">API Key</span><br>
                  <code style="display:block;background-color:#f1f5f9;border:1px solid #cbd5e1;border-radius:6px;padding:10px 12px;font-size:13px;color:#334155;word-break:break-all;margin-top:4px;">{api_key}</code>
                </td>
              </tr>
            </table>

            <!-- CTA Button -->
            <table width="100%" cellpadding="0" cellspacing="0" style="margin-bottom:24px;">
              <tr>
                <td align="center">
                  <a href="{invitation_link}" target="_blank" style="display:inline-block;background-color:#2563eb;color:#ffffff;text-decoration:none;padding:14px 36px;border-radius:8px;font-size:15px;font-weight:600;text-align:center;">点击完成注册</a>
                </td>
              </tr>
            </table>

            <p style="color:#9ca3af;font-size:12px;line-height:1.5;margin:0;">
              如果按钮无法点击，请复制以下链接粘贴到浏览器中打开：<br>
              <a href="{invitation_link}" style="color:#2563eb;word-break:break-all;font-size:12px;">{invitation_link}</a>
            </p>
          </td>
        </tr>
        <!-- Footer -->
        <tr>
          <td style="background-color:#f8fafc;border-top:1px solid #e2e8f0;padding:16px 24px;text-align:center;">
            <p style="color:#94a3b8;font-size:11px;margin:0;">
              此邮件由 Litellm Admin 自动发送 · 请勿回复
            </p>
          </td>
        </tr>
      </table>
    </td>
  </tr>
</table>
</body>
</html>"#,
        username = username,
        user_email = user_email,
        invitation_link = invitation_link,
        api_key = api_key,
    )
}
