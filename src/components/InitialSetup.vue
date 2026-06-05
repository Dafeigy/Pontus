<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import Button from "@/components/ui/button/Button.vue";
import Input from "@/components/ui/input/Input.vue";
import Label from "@/components/ui/label/Label.vue";
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@/components/ui/card";
import { toast } from "vue-sonner";
import type { AppConfig } from "@/stores/app";
import { IconArrowRight, IconCheck } from "@tabler/icons-vue";

const emit = defineEmits<{
  complete: [config: AppConfig];
}>();

const step = ref(1);
const loading = ref(false);

// Step 1 — required
const apiKey = ref("");
const litellmHost = ref("http://localhost:4000");

// Step 2 — optional
const smtpHost = ref("");
const smtpPort = ref("465");
const smtpSenderEmail = ref("");
const smtpUsername = ref("");
const smtpPassword = ref("");

function buildConfig(): AppConfig {
  return {
    apiKey: apiKey.value.trim(),
    litellmHost: litellmHost.value.trim() || "http://localhost:4000",
    smtpHost: smtpHost.value.trim(),
    smtpPort: Number(smtpPort.value) || 465,
    smtpSenderEmail: smtpSenderEmail.value.trim(),
    smtpUsername: smtpUsername.value.trim(),
    smtpPassword: smtpPassword.value,
    theme: "light",
  };
}

function goToStep2() {
  if (!litellmHost.value.trim()) {
    toast.error("请输入 Litellm 服务地址");
    return;
  }
  if (!apiKey.value.trim()) {
    toast.error("请输入 API Key");
    return;
  }
  step.value = 2;
}

async function handleSave() {
  loading.value = true;
  try {
    const config = buildConfig();
    await invoke("save_config_cmd", { config });
    toast.success("配置已保存");
    emit("complete", config);
  } catch (e) {
    toast.error(`保存失败: ${e}`);
  } finally {
    loading.value = false;
  }
}

async function handleSkip() {
  loading.value = true;
  try {
    const config = buildConfig();
    await invoke("save_config_cmd", { config });
    toast.success("配置已保存");
    toast.info("SMTP 未配置，可稍后在设置页面完成", { duration: 5000 });
    emit("complete", config);
  } catch (e) {
    toast.error(`保存失败: ${e}`);
  } finally {
    loading.value = false;
  }
}
</script>

<template>
  <div class="flex h-screen items-center justify-center bg-background p-4">
    <Card class="w-full max-w-md">
      <CardHeader class="text-center">
        <div class="mx-auto mb-3 flex h-12 w-12 items-center justify-center rounded-xl bg-secondary">
          <span class="text-xl">🧊</span>
        </div>
        <CardTitle class="text-xl">欢迎使用 Pontus</CardTitle>
        <CardDescription>
          {{ step === 1 ? "请先配置 Litellm 连接" : "配置 SMTP 邮件服务" }}
        </CardDescription>
      </CardHeader>

      <!-- Step indicators -->
      <div class="mb-6 flex items-center justify-center gap-2 px-6">
        <span
          class="flex h-7 w-7 items-center justify-center rounded-full text-xs font-medium"
          :class="step === 1 ? 'bg-primary text-primary-foreground' : 'bg-primary/15 text-primary'"
        >1</span>
        <span class="h-px w-8" :class="step === 1 ? 'bg-border' : 'bg-primary/40'" />
        <span
          class="flex h-7 w-7 items-center justify-center rounded-full text-xs font-medium"
          :class="step === 2 ? 'bg-primary text-primary-foreground' : 'bg-muted text-muted-foreground'"
        >2</span>
      </div>

      <CardContent class="space-y-4">
        <!-- Step 1: Litellm connection -->
        <div v-if="step === 1" class="space-y-4">
          <div class="space-y-2">
            <Label for="litellm-host">Litellm 服务地址</Label>
            <Input
              id="litellm-host"
              v-model="litellmHost"
              placeholder="http://localhost:4000"
              @keyup.enter="goToStep2"
            />
          </div>
          <div class="space-y-2">
            <Label for="apikey">API Key</Label>
            <Input
              id="apikey"
              v-model="apiKey"
              type="password"
              placeholder="sk-..."
              @keyup.enter="goToStep2"
            />
            <p class="text-xs text-muted-foreground">
              此 Key 将安全存储在本机
            </p>
          </div>

          <Button class="w-full" @click="goToStep2">
            下一步
            <IconArrowRight class="ml-2 h-4 w-4" />
          </Button>
        </div>

        <!-- Step 2: SMTP -->
        <div v-else class="space-y-4">
          <div class="grid grid-cols-2 gap-3">
            <div class="space-y-2">
              <Label for="smtp-host">SMTP 服务器</Label>
              <Input
                id="smtp-host"
                v-model="smtpHost"
                placeholder="smtp.example.com"
                :disabled="loading"
              />
            </div>
            <div class="space-y-2">
              <Label for="smtp-port">端口</Label>
              <Input
                id="smtp-port"
                v-model="smtpPort"
                type="number"
                placeholder="465"
                :disabled="loading"
              />
            </div>
          </div>
          <div class="space-y-2">
            <Label for="smtp-sender">发送者邮箱</Label>
            <Input
              id="smtp-sender"
              v-model="smtpSenderEmail"
              type="email"
              placeholder="noreply@example.com"
              :disabled="loading"
            />
          </div>
          <div class="grid grid-cols-2 gap-3">
            <div class="space-y-2">
              <Label for="smtp-username">SMTP 用户名</Label>
              <Input
                id="smtp-username"
                v-model="smtpUsername"
                placeholder="user@example.com"
                :disabled="loading"
              />
            </div>
            <div class="space-y-2">
              <Label for="smtp-password">SMTP 密码</Label>
              <Input
                id="smtp-password"
                v-model="smtpPassword"
                type="password"
                placeholder="••••••••"
                :disabled="loading"
              />
            </div>
          </div>

          <div class="flex gap-3 pt-2">
            <Button
              variant="outline"
              class="flex-1"
              :disabled="loading"
              @click="handleSkip"
            >
              {{ loading ? "保存中..." : "跳过" }}
            </Button>
            <Button
              class="flex-1"
              :disabled="loading"
              @click="handleSave"
            >
              <IconCheck v-if="!loading" class="mr-2 h-4 w-4" />
              {{ loading ? "保存中..." : "完成设置" }}
            </Button>
          </div>

          <p class="text-center text-xs text-muted-foreground">
            可稍后在设置页面配置 SMTP
          </p>
        </div>
      </CardContent>
    </Card>
  </div>
</template>
