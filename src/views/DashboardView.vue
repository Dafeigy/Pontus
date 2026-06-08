<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import Button from "@/components/ui/button/Button.vue";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import { Table, TableHeader, TableBody, TableRow, TableHead, TableCell } from "@/components/ui/table";
import { toast } from "vue-sonner";
import { IconRefresh, IconUsers } from "@tabler/icons-vue";
import { useCachedFetch } from "@/composables/useCachedFetch";

interface UserInfo {
  user_id: string;
  user_email: string | null;
  user_alias: string | null;
  user_role: string | null;
  spend: number | null;
  key_count: number | null;
  created_at: string | null;
}

interface UserListResponse {
  users: UserInfo[];
  total: number;
  page: number;
  page_size: number;
  total_pages: number;
}

// Cached fetch — only caches the first page; subsequent pages fetch fresh
const usersCache = useCachedFetch<UserListResponse>(
  `dashboard_users_page_1`,
  () => invoke("list_users", { page: 1, pageSize: pageSize })
);

const loading = ref(false);
const users = ref<UserInfo[]>([]);
const total = ref(0);
const page = ref(1);
const totalPages = ref(1);
const pageSize = 25;

const roleLabels: Record<string, string> = {
  proxy_admin: "网关管理员",
  proxy_admin_viewer: "审计管理员",
  internal_user: "普通用户",
  internal_user_viewer: "普通只读用户",
};

function formatDate(dateStr: string | null): string {
  if (!dateStr) return "-";
  const d = new Date(dateStr);
  return d.toLocaleString("zh-CN", {
    year: "numeric",
    month: "2-digit",
    day: "2-digit",
    hour: "2-digit",
    minute: "2-digit",
  });
}

function roleLabel(role: string | null): string {
  return role ? (roleLabels[role] ?? role) : "-";
}

async function loadUsers(force = false) {
  loading.value = true;
  try {
    const result = await usersCache.fetch(force);
    users.value = result.users;
    total.value = result.total;
    totalPages.value = result.total_pages;
  } catch (e) {
    toast.error(`加载用户列表失败: ${e}`);
  } finally {
    loading.value = false;
  }
}

async function refreshUsers() {
  await loadUsers(true);
}

function prevPage() {
  if (page.value > 1) {
    page.value--;
    loadUsers();
  }
}

function nextPage() {
  if (page.value < totalPages.value) {
    page.value++;
    loadUsers();
  }
}

onMounted(() => {
  loadUsers();
});
</script>

<template>
  <div class="space-y-6">
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-2">
        <IconUsers class="h-5 w-5 text-muted-foreground" />
        <h2 class="text-lg font-semibold">用户看板</h2>
        <span class="text-sm text-muted-foreground">（共 {{ total }} 个用户）</span>
      </div>
      <Button variant="outline" size="sm" :disabled="loading" @click="refreshUsers">
        <IconRefresh :class="['mr-2 h-4 w-4', loading && 'animate-spin']" />
        刷新
      </Button>
    </div>

    <Card>
      <CardHeader class="sr-only">
        <CardTitle>用户列表</CardTitle>
      </CardHeader>
      <CardContent class="p-0">
        <div v-if="loading && users.length === 0" class="space-y-2 p-4">
          <Skeleton v-for="i in 5" :key="i" class="h-10 w-full" />
        </div>
        <Table v-else>
          <TableHeader>
            <TableRow>
              <TableHead>用户名</TableHead>
              <TableHead>邮箱</TableHead>
              <TableHead>角色</TableHead>
              <TableHead>Key 数量</TableHead>
              <TableHead>消费 ($)</TableHead>
              <TableHead>创建时间</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            <TableRow v-for="u in users" :key="u.user_id">
              <TableCell class="font-medium">{{ u.user_alias || "-" }}</TableCell>
              <TableCell>{{ u.user_email || "-" }}</TableCell>
              <TableCell>
                <span class="inline-block rounded-full bg-muted px-2 py-0.5 text-xs">
                  {{ roleLabel(u.user_role) }}
                </span>
              </TableCell>
              <TableCell>{{ u.key_count ?? 0 }}</TableCell>
              <TableCell>{{ (u.spend ?? 0).toFixed(4) }}</TableCell>
              <TableCell class="text-xs text-muted-foreground">
                {{ formatDate(u.created_at) }}
              </TableCell>
            </TableRow>
            <TableRow v-if="users.length === 0">
              <TableCell colspan="6" class="py-8 text-center text-muted-foreground">
                暂无用户数据
              </TableCell>
            </TableRow>
          </TableBody>
        </Table>

        <!-- Pagination -->
        <div v-if="totalPages > 1" class="flex items-center justify-between border-t px-4 py-3">
          <span class="text-sm text-muted-foreground">
            第 {{ page }} / {{ totalPages }} 页
          </span>
          <div class="flex gap-2">
            <Button variant="outline" size="sm" :disabled="page <= 1" @click="prevPage">
              上一页
            </Button>
            <Button variant="outline" size="sm" :disabled="page >= totalPages" @click="nextPage">
              下一页
            </Button>
          </div>
        </div>
      </CardContent>
    </Card>
  </div>
</template>
