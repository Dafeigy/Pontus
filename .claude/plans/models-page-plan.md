# Plan: 新增模型页面

## 变更文件清单

### 1. `src-tauri/src/commands/litellm.rs`
新增：
- `AccessGroup` struct（反序列化 response）
- `ModelTestResult` struct（延迟、状态、模型名）
- `list_access_groups_internal()` — GET `/v1/access_group`
- `test_model_internal()` — POST `/v1/chat/completions`，记录耗时，判断 `content` 或 `reasoning_content` 非空即成功
- `list_access_groups` (tauri command, async wrapper)
- `test_model` (tauri command, async wrapper)

### 2. `src-tauri/src/lib.rs`
- 注册新命令 `list_access_groups`、`test_model`
- 导入新命令

### 3. `src/router/index.ts`
- 新增 `/models` 路由 → `ModelsView.vue`

### 4. `src/App.vue`
- `navMain` 中新增 "Models" 项（图标: `IconBoxMultiple`）

### 5. `src/views/ModelsView.vue`（新建）
- 调用 `list_access_groups` 获取 access_groups 列表
- 以 Card 分组，每组显示 `access_group_name` 和内部的 model 列表
- 每行 model 右侧有测试按钮（播放图标），点击后调用 `test_model`，显示延迟/状态
- 加载态用 Skeleton，空态提示
