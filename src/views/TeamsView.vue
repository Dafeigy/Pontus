<script setup lang="ts">
import { onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import Button from "@/components/ui/button/Button.vue";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import { Table, TableHeader, TableBody, TableRow, TableHead, TableCell } from "@/components/ui/table";
import { toast } from "vue-sonner";
import { IconRefresh, IconUsersGroup } from "@tabler/icons-vue";
import { useCachedFetch } from "@/composables/useCachedFetch";

interface TeamInfo {
  team_alias: string;
  team_id: string;
  members: unknown[];
  keys: unknown[];
  spend: number | null;
}

const teamsCache = useCachedFetch<TeamInfo[]>(
  "teams_list",
  () => invoke("list_teams") as Promise<TeamInfo[]>,
);

const { data: teams, loading, fetch: loadTeams, forceRefresh: refreshTeams } = teamsCache;

onMounted(() => {
  loadTeams().catch((e) => toast.error(`加载团队列表失败: ${e}`));
});

async function onRefresh() {
  try {
    await refreshTeams();
  } catch (e) {
    toast.error(`刷新失败: ${e}`);
  }
}

function copyId(id: string) {
  navigator.clipboard.writeText(id);
  toast.success("已复制 Team ID");
}
</script>

<template>
  <div class="space-y-6">
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-2">
        <IconUsersGroup class="h-5 w-5 text-muted-foreground" />
        <h2 class="text-lg font-semibold">团队列表</h2>
        <span class="text-sm text-muted-foreground">（共 {{ teams?.length ?? 0 }} 个团队）</span>
      </div>
      <Button variant="outline" size="sm" :disabled="loading" @click="onRefresh">
        <IconRefresh :class="['mr-2 h-4 w-4', loading && 'animate-spin']" />
        刷新
      </Button>
    </div>

    <Card>
      <CardHeader class="sr-only">
        <CardTitle>团队列表</CardTitle>
      </CardHeader>
      <CardContent class="p-0">
        <div v-if="loading && !teams" class="space-y-2 p-4">
          <Skeleton v-for="i in 5" :key="i" class="h-10 w-full" />
        </div>
        <Table v-else>
          <TableHeader>
            <TableRow>
              <TableHead>团队名称</TableHead>
              <TableHead>Team ID</TableHead>
              <TableHead>成员数</TableHead>
              <TableHead>Key 数量</TableHead>
              <TableHead>消费 ($)</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            <TableRow v-for="team in teams" :key="team.team_id">
              <TableCell class="font-medium">{{ team.team_alias }}</TableCell>
              <TableCell>
                <span
                  class="font-mono text-xs text-muted-foreground cursor-pointer hover:text-foreground transition-colors"
                  :title="team.team_id"
                  @click="copyId(team.team_id)"
                >
                  {{ team.team_id.slice(0, 12) }}...
                </span>
              </TableCell>
              <TableCell>{{ team.members?.length ?? 0 }}</TableCell>
              <TableCell>{{ team.keys?.length ?? 0 }}</TableCell>
              <TableCell>{{ (team.spend ?? 0).toFixed(4) }}</TableCell>
            </TableRow>
            <TableRow v-if="!teams || teams.length === 0">
              <TableCell colspan="5" class="py-8 text-center text-muted-foreground">
                暂无团队数据
              </TableCell>
            </TableRow>
          </TableBody>
        </Table>
      </CardContent>
    </Card>
  </div>
</template>
