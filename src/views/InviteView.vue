<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import Button from "@/components/ui/button/Button.vue";
import Input from "@/components/ui/input/Input.vue";
import Label from "@/components/ui/label/Label.vue";
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@/components/ui/card";
import { Select, SelectContent, SelectGroup, SelectItem, SelectItemText, SelectTrigger, SelectValue } from "@/components/ui/select";
import { toast } from "vue-sonner";
import { IconSend, IconLoader2, IconUserPlus, IconMail } from "@tabler/icons-vue";

const userAlias = ref("");
const userEmail = ref("");
const userRole = ref("internal_user_viewer");
const step = ref<"form" | "sending">("form");
const stepLabel = ref("");

const roleOptions = [
  { value: "proxy_admin", label: "网关管理员 (proxy_admin)" },
  { value: "proxy_admin_viewer", label: "审计管理员 (proxy_admin_viewer)" },
  { value: "internal_user", label: "普通用户 (internal_user)" },
  { value: "internal_user_viewer", label: "受限用户 (internal_user_viewer)" },
];

async function handleInvite() {
  if (!userEmail.value.trim()) {
    toast.error("请输入用户邮箱");
    return;
  }
  if (!userAlias.value.trim()) {
    toast.error("请输入用户名");
    return;
  }

  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  if (!emailRegex.test(userEmail.value.trim())) {
    toast.error("请输入有效的邮箱地址");
    return;
  }

  step.value = "sending";

  // Step 1: Create user via Litellm API
  stepLabel.value = "正在创建用户...";
  let userId: string;
  let apiKey: string;
  try {
    const result: any = await invoke("create_user", {
      userEmail: userEmail.value.trim(),
      userAlias: userAlias.value.trim(),
      userRole: userRole.value,
      keyAlias: `${userAlias.value.trim()}-key`,
    });
    userId = result.user_id;
    apiKey = result.key;
    toast.success(`用户创建成功 (ID: ${userId})`);
  } catch (e) {
    toast.error(`用户创建失败: ${e}`);
    step.value = "form";
    return;
  }

  // Step 2: Send invitation email
  stepLabel.value = "正在发送邀请邮件...";
  try {
    await invoke("complete_invitation", {
      userId,
      userEmail: userEmail.value.trim(),
      userAlias: userAlias.value.trim(),
      apiKey,
    });
    toast.success("邀请邮件已发送");
    userAlias.value = "";
    userEmail.value = "";
  } catch (e) {
    toast.error(`邮件发送失败: ${e}`);
  }

  step.value = "form";
}
</script>

<template>
  <div class="flex h-full items-start justify-center px-4 pt-12">
    <Card class="w-full max-w-lg">
      <CardHeader class="text-center">
        <div class="mx-auto mb-3 flex h-10 w-10 items-center justify-center rounded-lg bg-primary/10">
          <IconSend class="h-5 w-5 text-primary" />
        </div>
        <CardTitle>邀请新用户</CardTitle>
        <CardDescription>
          填写用户信息，系统将自动创建账号并发送邀请邮件
        </CardDescription>
      </CardHeader>

      <CardContent class="space-y-4">
        <!-- Username -->
        <div class="space-y-2">
          <Label for="alias">用户名</Label>
          <Input
            id="alias"
            v-model="userAlias"
            placeholder="请输入用户备注名"
            :disabled="step === 'sending'"
          />
        </div>

        <!-- Email -->
        <div class="space-y-2">
          <Label for="email">用户邮箱</Label>
          <Input
            id="email"
            v-model="userEmail"
            type="email"
            placeholder="user@example.com"
            :disabled="step === 'sending'"
            @keyup.enter="handleInvite"
          />
        </div>

        <!-- Role -->
        <div class="space-y-2 ">
          <Label>用户角色</Label>
          <Select v-model="userRole" :disabled="step === 'sending'" >
            <SelectTrigger class="w-full">
              <SelectValue placeholder="选择用户角色" />
            </SelectTrigger>
            <SelectContent>
              <SelectGroup>
                <SelectItem
                  v-for="opt in roleOptions"
                  :key="opt.value"
                  :value="opt.value"
                >
                  <SelectItemText>{{ opt.label }}</SelectItemText>
                </SelectItem>
              </SelectGroup>
            </SelectContent>
          </Select>
        </div>

        <!-- Steps indicator -->
        <div v-if="step === 'sending'" class="space-y-2 rounded-md bg-muted/50 p-3">
          <div class="flex items-center gap-2 text-sm">
            <IconUserPlus class="h-4 w-4 text-muted-foreground" />
            <span class="text-muted-foreground">步骤 1/2：创建 Litellm 用户</span>
          </div>
          <div class="flex items-center gap-2 text-sm">
            <IconMail class="h-4 w-4 text-muted-foreground" />
            <span class="text-muted-foreground">步骤 2/2：发送邀请邮件</span>
          </div>
          <p class="text-xs text-muted-foreground">{{ stepLabel }}</p>
        </div>

        <!-- Submit -->
        <Button class="w-full" size="lg" :disabled="step === 'sending'" @click="handleInvite">
          <IconLoader2 v-if="step === 'sending'" class="mr-2 h-4 w-4 animate-spin" />
          <IconSend v-else class="mr-2 h-4 w-4" />
          {{ step === "sending" ? "处理中..." : "发送邀请" }}
        </Button>
      </CardContent>
    </Card>
  </div>
</template>
