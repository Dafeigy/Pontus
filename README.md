# Tauri + Vue + TypeScript + Tailwindcssv4 + ShadCN-Vue

This template should help get you started developing with Vue 3 and TypeScript in Vite. The template uses Vue 3 `<script setup>` SFCs, check out the [script setup docs](https://v3.vuejs.org/api/sfc-script-setup.html#sfc-script-setup) to learn more.

## Recommended IDE Setup

1. Using Tailwindcss v4.

```bash
npm install tailwindcss @tailwindcss/vite
```

Then setup Tailwindcss v4. Add the @tailwindcss/vite plugin to your Vite configuration:
```diff
import { defineConfig } from 'vite'
+ import tailwindcss from '@tailwindcss/vite'

export default defineConfig({
  plugins: [
    vue(),
+    tailwindcss(),
  ],
})
```

After that, Add an @import to your CSS file that imports Tailwind CSS. In this template, it is located in `src\assets\main.css`.

2. Setup ShadCN-Vue

