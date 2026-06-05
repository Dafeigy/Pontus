## 用户故事

Litellm自带的webui可以帮助运维人员进行用户注册、生成注册用户的邀请链接，并在理论上可以通过电子邮件的形式发送相关信息给注册用户留下的电子邮箱。然而，这样对普通的非技术运维人员来讲还是有点麻烦，加上litellm的邮件发送功能存在奇怪的Bug，短时间内无法修复。需要构建一个基于Tauri2的本地桌面端程序，使用Litellm提供的API进行用户注册、发送邮件等操作。

## 需求

1. 技术栈采用Tauri2+Vue3，组件使用ShadCN-Vue。有提供shadcn的MCP服务。
2. 从结构上看，共有三个界面：主页面暂时留空，设置页面Settings用于完成相关设置，以及一个看板页Dashboard，用于查看系统已有用户信息。当前模板有一个Invite User的按钮，这个按钮点击后会有一个dialog组件弹出，用户在里面填写注册用户的邮箱、用户别名、用户角色后，可以点击邀请按钮邀请用户。
3. 初次启动会需要设置`API_Key`，它是后续API请求中需要的鉴权参数，一旦设置，除非在设置页面进行重置，否则无法更改。`API_Key`存储到本地文件系统中就可以。
4. 设置页面需要设置的内容包括：
   1. SMTP的服务器地址`SMTP_HOST`
   2. SMTP的服务端口`SMTP_PORT`
   3. 发送者的电子邮箱`SMTP_SENDER_EMAIL`
   4. SMTP的用户名`SMTP_USERNAME`
   5. SMTP的密码`SMTP_PASSWORD`
   6. Litellm服务的运行url`LITELLM_HOST`
5. 设置页面还需要支持亮暗模式的切换。期望是点击按钮从自动、暗黑、明亮模式轮流切换。
6. 设置页面需要有重置API Key的按钮。
7. 邀请页面的dialog采用极简设计，只需要输入用户提供的电子邮箱地址，点击邀请按钮后会使用设置页面中设置的SMTP发送一封模板html的电子邮件给用户。SMTP的邮件发送使用Rust后端，默认端口465，SSL方式。参考的模板如下：
```rust
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

```

## 接口

### 新建用户

邀请新用户的curl命令如下所示。其中：

- `API_KEY`: 设置的API Key。
- `USER_ROLE`:用户角色，枚举类型。均为字符串，值为:`['proxy_admin','proxy_admin_viewer','internal_user','internal_user_viewer']`,默认为`internal_user_viewer`。他们分别对应网关管理员、审计管理员、普通用户、普通只读用户。
- `USER_ALAS`: 用户备注名（必填）
- `USER_EMAIL`: 用户电子邮箱（必填）

```bash
curl -X POST "http://<LITELLM_HOST>/user/new" -H "Content-Type: application/json" -H "Authorization: Bearer <API_KEY>"      -d '{
     "user_email": "<USER_EMAIL>",
     "user_alias": "<USER_ALAS>",
     "user_role": "<USER_ROLE>"
 }'

# example
curl -X POST "http://example.com/user/new" -H "Content-Type: application/json" -H "Authorization: Bearer sk-1234" -d '{
     "user_email": "1748822508@qq.com",
     "user_alias": "hello",
     "user_role": "internal_user_viewer"
 }'
```

创建成功响应如下：

```json
{
  "key_alias": null,
  "duration": null,
  "models": [],
  "spend": 0.0,
  "max_budget": null,
  "user_id": "35e38eff-fac2-487d-bb91-ab710bcae2a2",
  "team_id": null,
  "agent_id": null,
  "max_parallel_requests": null,
  "metadata": {},
  "tpm_limit": null,
  "rpm_limit": null,
  "budget_duration": null,
  "budget_limits": null,
  "allowed_cache_controls": [],
  "config": {},
  "permissions": {},
  "model_max_budget": {},
  "model_rpm_limit": null,
  "model_tpm_limit": null,
  "guardrails": null,
  "policies": null,
  "prompts": null,
  "blocked": null,
  "aliases": {},
  "object_permission": null,
  "key": "sk-nneaY0HnWNX5_q574unLyw",
  "budget_id": null,
  "tags": null,
  "enforced_params": null,
  "allowed_routes": [],
  "allowed_passthrough_routes": null,
  "allowed_vector_store_indexes": null,
  "rpm_limit_type": null,
  "tpm_limit_type": null,
  "router_settings": {
    "routing_strategy_args": null,
    "routing_strategy": null,
    "routing_groups": null,
    "model_group_retry_policy": null,
    "model_group_affinity_config": null,
    "allowed_fails": null,
    "cooldown_time": null,
    "num_retries": null,
    "timeout": null,
    "max_retries": null,
    "retry_after": null,
    "fallbacks": null,
    "context_window_fallbacks": null,
    "model_group_alias": {}
  },
  "access_group_ids": [],
  "key_name": "sk-...nLyw",
  "expires": null,
  "token_id": null,
  "organization_id": null,
  "project_id": null,
  "litellm_budget_table": null,
  "token": null,
  "created_by": null,
  "updated_by": null,
  "created_at": "2026-06-02T02:54:49.084000Z",
  "updated_at": "2026-06-02T02:54:49.084000Z",
  "user_email": "1748822508@qq.com",
  "user_role": "internal_user_viewer",
  "teams": null,
  "user_alias": "hello"
}
```

### 生成用户邀请链接

用户邀请链接格式为`http://<LITELLM_HOST>/ui?invitation_id=<INVITATION_ID>`。

获取`INVITATION_ID`的请求如下。其中：

- `USER_ID`为新建用户成功时返回的`user_id`字段。

```bash
# example
curl -X POST "http://example.com/invitation/new" -H "Authorization: Bearer sk-1234" -H "Content-Type: application/json" -d '{"user_id": "35e38eff-fac2-487d-bb91-ab710bcae2a2"}'
```

请求成功的响应如下：

```bash
{
  "id": "9ce4715a-3a5a-49dc-a83c-6c424f3cf368",
  "user_id": "35e38eff-fac2-487d-bb91-ab710bcae2a2",
  "is_accepted": false,
  "accepted_at": null,
  "expires_at": "2026-06-09T03:00:17.275000Z",
  "created_at": "2026-06-02T03:00:17.275000Z",
  "created_by": "default_user_id",
  "updated_at": "2026-06-02T03:00:17.275000Z",
  "updated_by": "default_user_id"
}
```

最终的邀请链接为`http://<LITELLM_HOST>/ui?invitation_id=<INVITATION_ID>`。`INVITATION_ID`实际为请求响应中的`id`字段。

### 获取所有用户信息

该步骤用于展示当前的用户情况。使用如下请求获取：

```bash
curl -X GET "http://<LITELLM_HOST>/user/list" -H "Authorization: Bearer ccr24202" -H "Content-Type: application/json" 

# example
curl -X GET "http://example.com/user/list" -H "Authorization: Bearer sk-1234" -H "Content-Type: application/json" 
```

请求成功的响应返回如下：

```json
{
  "users": [
    {
      "user_id": "35e38eff-fac2-487d-bb91-ab710bcae2a2",
      "max_budget": null,
      "spend": 0.0,
      "model_max_budget": {},
      "model_spend": {},
      "user_email": "1748822508@qq.com",
      "user_alias": "hello",
      "models": [],
      "tpm_limit": null,
      "rpm_limit": null,
      "user_role": "internal_user_viewer",
      "organization_memberships": null,
      "teams": [],
      "sso_user_id": null,
      "budget_duration": null,
      "budget_reset_at": null,
      "metadata": {},
      "created_at": "2026-06-02T02:54:49.078000Z",
      "updated_at": "2026-06-02T02:54:49.078000Z",
      "object_permission": null,
      "key_count": 1
    },
    {
      "user_id": "a9edd82e-981d-4e59-8e0e-1d628472833c",
      "max_budget": null,
      "spend": 0.0,
      "model_max_budget": {},
      "model_spend": {},
      "user_email": "1070642765@qq.com",
      "user_alias": null,
      "models": [],
      "tpm_limit": null,
      "rpm_limit": null,
      "user_role": "proxy_admin",
      "organization_memberships": null,
      "teams": [],
      "sso_user_id": null,
      "budget_duration": null,
      "budget_reset_at": null,
      "metadata": {},
      "created_at": "2026-05-24T12:20:07.174000Z",
      "updated_at": "2026-05-26T03:47:26.075000Z",
      "object_permission": null,
      "key_count": 2
    },
    {
      "user_id": "default_user_id",
      "max_budget": null,
      "spend": 0.0,
      "model_max_budget": {},
      "model_spend": {},
      "user_email": null,
      "user_alias": null,
      "models": [],
      "tpm_limit": null,
      "rpm_limit": null,
      "user_role": "proxy_admin",
      "organization_memberships": null,
      "teams": [],
      "sso_user_id": null,
      "budget_duration": null,
      "budget_reset_at": null,
      "metadata": {},
      "created_at": "2026-05-08T07:45:23.244000Z",
      "updated_at": "2026-06-02T01:12:34.462000Z",
      "object_permission": null,
      "key_count": 0
    }
  ],
  "total": 3,
  "page": 1,
  "page_size": 25,
  "total_pages": 1
}
```



## 其他信息

发送的电子邮件html模板请参考下图的右半部分。Accept按钮对应跳转用户邀请链接。

![](F:\Prog\Buddy\Litellm-manager\email_template.png)

### 界面风格偏好

another admin key：sk-ax0R405r2h5nsvA163MatQ