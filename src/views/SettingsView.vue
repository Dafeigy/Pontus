<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import Button from "@/components/ui/button/Button.vue";
import Input from "@/components/ui/input/Input.vue";
import Label from "@/components/ui/label/Label.vue";
import { Switch } from "@/components/ui/switch";
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@/components/ui/card";
import { toast } from "vue-sonner";
import type { AppConfig } from "@/stores/app";
import { useAppStore } from "@/stores/app";
import { IconDeviceFloppy, IconKey, IconTrash } from "@tabler/icons-vue";

const store = useAppStore();

const apiKey = ref("");
const litellmHost = ref("http://localhost:4000");
const smtpHost = ref("");
const smtpPort = ref("465");
const smtpSenderEmail = ref("");
const smtpUsername = ref("");
const smtpPassword = ref("");
const isDark = ref(false);
const saving = ref(false);
const resetting = ref(false);

onMounted(async () => {
  try {
    const config: AppConfig = await invoke("get_config_cmd");
    apiKey.value = config.apiKey || "";
    litellmHost.value = config.litellmHost || "http://localhost:4000";
    smtpHost.value = config.smtpHost || "";
    smtpPort.value = String(config.smtpPort || 465);
    smtpSenderEmail.value = config.smtpSenderEmail || "";
    smtpUsername.value = config.smtpUsername || "";
    smtpPassword.value = config.smtpPassword || "";
    isDark.value = document.documentElement.classList.contains("dark");
  } catch (e) {
    toast.error(`加载配置失败: ${e}`);
  }
});

async function saveConfig() {
  saving.value = true;
  try {
    const config: AppConfig = {
      apiKey: apiKey.value,
      litellmHost: litellmHost.value,
      smtpHost: smtpHost.value,
      smtpPort: Number(smtpPort.value) || 465,
      smtpSenderEmail: smtpSenderEmail.value,
      smtpUsername: smtpUsername.value,
      smtpPassword: smtpPassword.value,
      theme: isDark.value ? "dark" : "light",
    };
    await invoke("save_config_cmd", { config });
    store.config = config;
    store.applyTheme(config.theme as "light" | "dark");
    toast.success("配置已保存");
  } catch (e) {
    toast.error(`保存失败: ${e}`);
  } finally {
    saving.value = false;
  }
}

async function resetApiKey() {
  if (!confirm("确定要重置 API Key 吗？此操作将清除已存储的 Key，需要重新设置。")) return;
  resetting.value = true;
  try {
    const config: AppConfig = await invoke("reset_api_key_cmd");
    apiKey.value = "";
    store.config = config;
    store.isInitialized = false;
    toast.success("API Key 已重置，请重新设置");
  } catch (e) {
    toast.error(`重置失败: ${e}`);
  } finally {
    resetting.value = false;
  }
}

function toggleTheme(val: boolean) {
  isDark.value = val;
  store.config.theme = val ? "dark" : "light";
  store.applyTheme(val ? "dark" : "light");
}
</script>

<template>
  <div class="mx-auto max-w-2xl space-y-6">
    <div>
      <h2 class="text-lg font-semibold">设置</h2>
      <p class="text-sm text-muted-foreground">配置 Litellm 连接和 SMTP 邮件服务</p>
    </div>

    <!-- Litellm API -->
    <Card>
      <CardHeader>
        <CardTitle class="flex items-center gap-2 text-base">
          <IconKey class="h-4 w-4" /> Litellm API 配置
        </CardTitle>
        <CardDescription>配置 Litellm 服务的连接信息</CardDescription>
      </CardHeader>
      <CardContent class="space-y-4">
        <div class="space-y-2">
          <Label for="litellm-host">Litellm 服务地址</Label>
          <Input id="litellm-host" v-model="litellmHost" placeholder="http://localhost:4000" />
        </div>
        <div class="space-y-2">
          <Label for="api-key">API Key</Label>
          <Input id="api-key" v-model="apiKey" type="password" placeholder="sk-..." />
        </div>
        <Button variant="destructive" size="sm" :disabled="resetting" @click="resetApiKey">
          <IconTrash class="mr-2 h-4 w-4" />
          {{ resetting ? "重置中..." : "重置 API Key" }}
        </Button>
      </CardContent>
    </Card>

    <!-- SMTP -->
    <Card>
      <CardHeader>
        <CardTitle class="text-base">✉️ SMTP 邮件配置</CardTitle>
        <CardDescription>配置用于发送邀请邮件的 SMTP 服务器</CardDescription>
      </CardHeader>
      <CardContent class="space-y-4">
        <div class="grid grid-cols-2 gap-4">
          <div class="space-y-2">
            <Label for="smtp-host">SMTP 服务器地址</Label>
            <Input id="smtp-host" v-model="smtpHost" placeholder="smtp.example.com" />
          </div>
          <div class="space-y-2">
            <Label for="smtp-port">SMTP 端口</Label>
            <Input id="smtp-port" v-model="smtpPort" type="number" placeholder="465" />
          </div>
        </div>
        <div class="space-y-2">
          <Label for="smtp-sender">发送者邮箱</Label>
          <Input id="smtp-sender" v-model="smtpSenderEmail" type="email" placeholder="noreply@example.com" />
        </div>
        <div class="grid grid-cols-2 gap-4">
          <div class="space-y-2">
            <Label for="smtp-username">SMTP 用户名（通常为完整邮箱地址）</Label>
            <Input id="smtp-username" v-model="smtpUsername" placeholder="user@example.com" />
          </div>
          <div class="space-y-2">
            <Label for="smtp-password">SMTP 密码</Label>
            <Input id="smtp-password" v-model="smtpPassword" type="password" placeholder="••••••••" />
          </div>
        </div>
      </CardContent>
    </Card>

    <!-- Theme -->
    <Card>
      <CardHeader>
        <CardTitle class="text-base">外观</CardTitle>
        <CardDescription>自定义应用的外观主题</CardDescription>
      </CardHeader>
      <CardContent>
        <div class="flex items-center justify-between">
          <div>
            <p class="text-sm font-medium">暗黑模式</p>
            <p class="text-xs text-muted-foreground">切换应用的亮色 / 暗色主题</p>
          </div>
          <Switch v-model="isDark" @update:model-value="toggleTheme" />
        </div>
      </CardContent>
    </Card>

    <!-- Save -->
    <div class="flex justify-end">
      <Button size="lg" :disabled="saving" @click="saveConfig">
        <IconDeviceFloppy class="mr-2 h-4 w-4" />
        {{ saving ? "保存中..." : "保存配置" }}
      </Button>
    </div>
  </div>
</template>
