# Tauri + Vue + TypeScript + Tailwindcssv4 + ShadCN-Vue

This template should help get you started developing with Vue 3 and TypeScript in Vite. The template uses Vue 3 `<script setup>` SFCs, check out the [script setup docs](https://v3.vuejs.org/api/sfc-script-setup.html#sfc-script-setup) to learn more.

## Recommand Setup

1. Change product name and other meta info. 
  ```diff
  # package.json
  -"name": "tauri-app",
  +"name": "App Name",
  ```
  In `stc-tauri/Cargo.toml`:
  ```diff
  [package]
  -name = "tauri-app"
  +name = "App Name"
  version = "0.1.0"
  description = "A Tauri App"
  - authors = ["you"]
  + authors = ["Author Name"]
  edition = "2021"
  ```

  In `src-tauri/tauri.conf.json`:
  ```diff
    "productName": "tauri-app",
    "version": "0.1.0",
  - "identifier": "com.cybersh1t.tauri-app",
  + "identifier": "com.yourname.app-name",
  ```

2. Follow best practice of shadcn-vue.

  To start with this part, if you are using claude code, run the following command in your project:

  ```bash
  npx shadcn-vue@latest mcp init --client claude

  ```

  Restart Claude Code and try the following prompts:

  - Show me all available components in the shadcn registry
  - Add the button, dialog and card components to my project
  - Create a contact form using components from the shadcn registry

  Note: You can use /mcp command in Claude Code to debug the MCP server.

3. Write requirements and let your coding agent to make a plan.