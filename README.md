<h1 align="center">
  <img src="src/assets/vue.svg" width="64" alt="Pontus" />
  <br>Pontus
</h1>

<p align="center">
  <strong>Litellm 运维桌面客户端</strong><br>
  基于 Tauri v2 + Vue 3 构建，提供用户管理、模型连通性测试与 LLM 对话 Playground。
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Tauri-2-FFC131?logo=tauri" />
  <img src="https://img.shields.io/badge/Vue-3.5-4FC08D?logo=vuedotjs" />
  <img src="https://img.shields.io/badge/TypeScript-5.6-3178C6?logo=typescript" />
  <img src="https://img.shields.io/badge/Rust-2021-000000?logo=rust" />
  <img src="https://img.shields.io/badge/Tailwind-4-06B6D4?logo=tailwindcss" />
</p>

---

## 功能

- **首次启动引导** — 配置 Litellm API 地址与密钥，可选 SMTP 邮件服务
- **用户看板** — 查看用户列表、消费额度、Key 数量，支持本地缓存
- **邀请用户** — 创建 Litellm 用户 → 生成邀请链接 → 自动发送邮件
- **模型列表** — 按 Access Group 分组展示模型，支持单个/一键批量连通性测试
- **Playground** — 选择模型，流式对话，Markdown 实时渲染
- **系统设置** — 修改 API 配置、SMTP 参数、深色/浅色主题切换
- **本地缓存** — GET 请求（用户列表、模型列表）自动存入 localStorage，每日自动刷新

## 技术栈

| 层 | 技术 |
|---|---|
| 桌面框架 | Tauri v2 |
| 前端 | Vue 3 + TypeScript + Vite |
| UI | shadcn-vue (reka-ui) + Tailwind CSS v4 |
| 状态管理 | Pinia + tauri-plugin-store (持久化) |
| 图表 | @unovis/vue |
| Rust HTTP | reqwest (blocking) |
| Rust 邮件 | lettre (SMTP) |
| 流式对话 | SSE → Tauri event emit → Vue listen |

## 项目结构

```
pontus/
├── src/                          # Vue 前端
│   ├── App.vue                   # 根组件 (加载屏 / 初始化引导 / 布局)
│   ├── main.ts                   # 入口，挂载 Pinia + Router
│   ├── router/index.ts           # Hash 路由配置
│   ├── stores/app.ts             # 全局应用状态 (主题、配置)
│   ├── composables/              # 组合式函数
│   │   └── useCachedFetch.ts     # 通用 localStorage 每日缓存
│   ├── views/                    # 页面视图
│   │   ├── DashboardView.vue     # 用户看板
│   │   ├── InviteView.vue        # 邀请用户
│   │   ├── ModelsView.vue        # 模型列表与连通性测试
│   │   ├── PlayGroundView.vue    # LLM 对话 Playground
│   │   ├── SettingsView.vue      # 系统设置
│   │   ├── TeamsView.vue         # 团队管理 (预留)
│   │   └── HomeView.vue          # 首页
│   ├── components/               # 组件
│   │   ├── InitialSetup.vue      # 首次启动两步向导
│   │   ├── AppSidebar.vue        # 侧边栏
│   │   ├── SiteHeader.vue        # 顶栏
│   │   ├── NavMain.vue           # 主导航
│   │   ├── NavUser.vue           # 用户区
│   │   └── ui/                   # shadcn-vue UI 原语 (50+)
│   └── assets/main.css           # Tailwind + CSS 变量主题
│
├── src-tauri/                    # Rust 后端
│   ├── Cargo.toml
│   ├── tauri.conf.json           # Tauri 配置
│   └── src/
│       ├── main.rs               # Windows 隐藏控制台
│       ├── lib.rs                # 插件注册 + 命令挂载
│       └── commands/
│           ├── config.rs         # 应用配置读写 (tauri-plugin-store)
│           ├── email.rs          # SMTP 邮件发送
│           ├── email_template.rs # 邀请邮件 HTML 模板
│           └── litellm/          # Litellm API 客户端
│               ├── mod.rs        # 公共工具 + re-export
│               ├── types.rs      # 数据结构定义
│               ├── users.rs      # 用户 CRUD / 邀请
│               └── models.rs     # 模型列表 / 测试 / 流式对话
```

## 快速开始

### 环境要求

- [Node.js](https://nodejs.org/) ≥ 18
- [Rust](https://www.rust-lang.org/tools/install) ≥ 1.70
- Windows: [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/) (C++ 桌面开发)

### 安装依赖

```bash
npm install
```

### 开发模式

```bash
# 仅前端 (浏览器)
npm run dev

# 完整 Tauri 桌面应用
npm run tauri dev
```

### 构建

```bash
npm run tauri build
```

产物位于 `src-tauri/target/release/bundle/`。

## Tauri 命令一览

| 命令 | 用途 |
|---|---|
| `is_initialized_cmd` | 检查是否已完成首次配置 |
| `get_config_cmd` / `save_config_cmd` / `reset_api_key_cmd` | 应用配置读写 |
| `create_user` / `generate_invitation` / `list_users` | Litellm 用户管理 |
| `list_access_groups` / `test_model` | 模型列表与连通性测试 |
| `chat_stream` | 流式对话 (SSE → event emit) |
| `send_invite_email` / `complete_invitation` / `invite_user` | 邮件邀请 |

## 配置存储

应用配置使用 tauri-plugin-store 持久化到本地文件，包含：
- Litellm 服务地址 + API Key
- SMTP 服务器 / 端口 / 用户名 / 密码 / 发件邮箱
- 主题偏好 (light / dark)
