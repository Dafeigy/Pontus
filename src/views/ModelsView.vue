<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import Button from "@/components/ui/button/Button.vue";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import { Badge } from "@/components/ui/badge";
import { toast } from "vue-sonner";
import { IconPlayerPlay, IconLoader2, IconRefresh, IconPlayerTrackNext, IconBoxModel } from "@tabler/icons-vue";
import { useCachedFetch } from "@/composables/useCachedFetch";

interface AccessGroup {
  access_group_id: string;
  access_group_name: string;
  access_model_names: string[];
  description: string | null;
  created_at: string | null;
}

interface ModelTestResult {
  model: string;
  success: boolean;
  latency_ms: number;
  message: string;
}

const groupsCache = useCachedFetch<AccessGroup[]>(
  "models_access_groups",
  () => invoke("list_access_groups")
);

const loading = ref(false);
const groups = ref<AccessGroup[]>([]);
const testingModels = ref<Set<string>>(new Set());
const testResults = ref<Map<string, ModelTestResult>>(new Map());
const testAllInProgress = ref(false);

const allModels = computed(() =>
  groups.value.flatMap((g) => g.access_model_names)
);

async function loadGroups(force = false) {
  loading.value = true;
  try {
    groups.value = await groupsCache.fetch(force);
  } catch (e) {
    toast.error(`加载模型列表失败: ${e}`);
  } finally {
    loading.value = false;
  }
}

async function refreshGroups() {
  await loadGroups(true);
}

async function testModel(model: string) {
  testingModels.value.add(model);
  try {
    const result: ModelTestResult = await invoke("test_model", { model });
    testResults.value.set(model, result);
  } catch (e) {
    testResults.value.set(model, {
      model,
      success: false,
      latency_ms: 0,
      message: `${e}`,
    });
  } finally {
    testingModels.value.delete(model);
  }
}

async function testAllModels() {
  testAllInProgress.value = true;
  const models = allModels.value;
  const batchSize = 5;
  for (let i = 0; i < models.length; i += batchSize) {
    const batch = models.slice(i, i + batchSize);
    await Promise.all(batch.map((m) => testModel(m)));
  }
  testAllInProgress.value = false;
}

function resultBadge(model: string): { variant: "default" | "destructive" | "secondary"; text: string } | null {
  const r = testResults.value.get(model);
  if (!r) return null;
  return r.success
    ? { variant: "default", text: `${r.latency_ms}ms` }
    : { variant: "destructive", text: "失败" };
}

onMounted(() => {
  loadGroups();
});
</script>

<template>
  <div class="space-y-6">
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-2">
        <IconBoxModel class="h-5 w-5 text-muted-foreground" />
        <h2 class="text-lg font-semibold">Models Groups</h2>
      </div>
      <div class="flex items-center gap-2">
        <Button
          variant="outline"
          size="sm"
          :disabled="testAllInProgress || loading"
          @click="testAllModels"
        >
          <IconPlayerTrackNext class="mr-1 h-4 w-4" />
          Test Connection
        </Button>
        <Button variant="outline" size="sm" :disabled="loading" @click="refreshGroups">
          <IconRefresh :class="['mr-2 h-4 w-4', loading && 'animate-spin']" />
          Refresh
        </Button>
      </div>
    </div>

    <!-- Loading -->
    <div v-if="loading" class="space-y-4">
      <Skeleton v-for="i in 3" :key="i" class="h-32 w-full" />
    </div>

    <!-- Empty -->
    <div v-else-if="groups.length === 0" class="flex items-center justify-center py-16 text-muted-foreground">
      暂无模型数据
    </div>

    <!-- Groups -->
    <div v-else class="space-y-4">
      <Card v-for="group in groups" :key="group.access_group_id">
        <CardHeader class="pb-3">
          <CardTitle class="text-base">{{ group.access_group_name }}</CardTitle>
        </CardHeader>
        <CardContent class="space-y-2">
          <div
            v-for="model in group.access_model_names"
            :key="model"
            class="flex items-center justify-between rounded-md border px-3 py-2"
          >
            <span class="text-sm font-mono">{{ model }}</span>
            <div class="flex items-center gap-3">
              <Badge
                v-if="resultBadge(model)"
                :variant="resultBadge(model)!.variant"
              >
                {{ resultBadge(model)!.text }}
              </Badge>
              <Button
                variant="ghost"
                size="sm"
                :disabled="testingModels.has(model)"
                @click="testModel(model)"
              >
                <IconLoader2 v-if="testingModels.has(model)" class="h-4 w-4 animate-spin" />
                <IconPlayerPlay v-else class="h-4 w-4" />
              </Button>
            </div>
          </div>
          <div v-if="group.access_model_names.length === 0" class="py-4 text-center text-sm text-muted-foreground">
            该分组下暂无模型
          </div>
        </CardContent>
      </Card>
    </div>
  </div>
</template>
