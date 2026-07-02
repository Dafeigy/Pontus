<script setup lang="ts">
import { ref, onMounted, onUnmounted, nextTick, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { useCachedFetch } from "@/composables/useCachedFetch";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectItemText,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import Textarea from "@/components/ui/textarea/Textarea.vue";
import { IconSend, IconTrash, IconBoxModel, IconSparkles, IconBrain } from "@tabler/icons-vue";

// --- Types ---

interface ChatMessage {
  role: "user" | "assistant" | "system";
  content: string;
  thinkContent: string;
}

interface StreamChunk {
  stream_id: string;
  content: string;
  think_content: string;
  done: boolean;
  error: string | null;
}

interface AccessGroup {
  access_group_id: string;
  access_group_name: string;
  access_model_names: string[];
  description?: string;
}

// --- Model list ---

const { data: accessGroups, fetch: fetchGroups } = useCachedFetch<AccessGroup[]>(
  "playground_models",
  async () => {
    const raw = await invoke("list_access_groups");
    return JSON.parse(JSON.stringify(raw)) as AccessGroup[];
  },
);

const modelOptions = ref<string[]>([]);

watch(
  accessGroups,
  (groups) => {
    if (!groups) return;
    const names = new Set<string>();
    for (const g of groups) {
      for (const m of g.access_model_names) {
        names.add(m);
      }
    }
    modelOptions.value = Array.from(names).sort();
  },
  { immediate: true },
);

const selectedModel = ref<string | null>(null);

// --- Chat state ---

const messages = ref<ChatMessage[]>([]);
const inputText = ref("");
const isStreaming = ref(false);
const currentStreamId = ref<string | null>(null);
let unlistenFn: UnlistenFn | null = null;

// --- DOM refs ---

const messagesContainer = ref<HTMLElement | null>(null);

// --- Event listener ---

onMounted(async () => {
  unlistenFn = await listen<StreamChunk>("chat-stream-chunk", (event) => {
    if (event.payload.stream_id !== currentStreamId.value) return;

    if (event.payload.error) {
      const lastMsg = messages.value[messages.value.length - 1];
      if (lastMsg && lastMsg.role === "assistant" && lastMsg.content === "" && lastMsg.thinkContent === "") {
        lastMsg.content = `❌ 错误: ${event.payload.error}`;
      } else {
        messages.value.push({
          role: "system",
          content: `❌ 错误: ${event.payload.error}`,
          thinkContent: "",
        });
      }
      finishStream();
      return;
    }

    if (event.payload.done) {
      finishStream();
      return;
    }

    const lastMsg = messages.value[messages.value.length - 1];
    if (lastMsg && lastMsg.role === "assistant") {
      // Accumulate reasoning/think content (from dedicated delta field)
      if (event.payload.think_content) {
        lastMsg.thinkContent += event.payload.think_content;
      }
      // Accumulate regular content
      if (event.payload.content) {
        lastMsg.content += event.payload.content;
      }
      // Parse inline <think> tags from content (for models that embed them)
      if (lastMsg.content) {
        const extracted = extractThinkBlocks(lastMsg.content);
        if (extracted.thinkText) {
          lastMsg.thinkContent += extracted.thinkText;
          lastMsg.content = extracted.cleanText;
        }
      }
      scrollToBottom();
    }
  });

  await fetchGroups();
});

onUnmounted(() => {
  unlistenFn?.();
});

// --- Functions ---

function scrollToBottom() {
  nextTick(() => {
    if (messagesContainer.value) {
      messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight;
    }
  });
}

function finishStream() {
  isStreaming.value = false;
  currentStreamId.value = null;
  scrollToBottom();
}

async function sendMessage() {
  const text = inputText.value.trim();
  if (!text || !selectedModel.value || isStreaming.value) return;

  inputText.value = "";

  messages.value.push({ role: "user", content: text, thinkContent: "" });
  messages.value.push({ role: "assistant", content: "", thinkContent: "" });
  scrollToBottom();

  const apiMessages = messages.value
    .filter((m) => m.role !== "system" && m.content !== "")
    .map((m) => ({ role: m.role, content: m.content }));

  isStreaming.value = true;
  currentStreamId.value = crypto.randomUUID();

  try {
    await invoke("chat_stream", {
      streamId: currentStreamId.value,
      model: selectedModel.value,
      messages: apiMessages,
    });
  } catch (e) {
    const lastMsg = messages.value[messages.value.length - 1];
    if (lastMsg && lastMsg.role === "assistant" && lastMsg.content === "") {
      lastMsg.content = `❌ 请求失败: ${e}`;
    }
    finishStream();
  }
}

function clearConversation() {
  messages.value = [];
  currentStreamId.value = null;
  isStreaming.value = false;
}

function handleKeydown(e: KeyboardEvent) {
  if (e.key === "Enter" && !e.shiftKey) {
    e.preventDefault();
    sendMessage();
  }
}

// --- Think-block extraction (for models that embed <think> tags in content) ---

function extractThinkBlocks(text: string): { cleanText: string; thinkText: string } {
  let thinkText = "";

  // Match complete <think>...</think> blocks only (both tags present)
  const fullThinkRegex = /<think>([\s\S]*?)<\/think>/g;
  const cleanText = text.replace(fullThinkRegex, (_match, inner: string) => {
    thinkText += inner;
    return "";
  });

  return { cleanText, thinkText };
}

// --- Markdown rendering ---

function renderContent(text: string): string {
  if (!text) return "";

  let html = text
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;");

  html = html.replace(
    /```(\w*)\n?([\s\S]*?)```/g,
    (_: string, _lang: string, code: string) => {
      return `<pre class="code-block"><code>${code.trimEnd()}</code></pre>`;
    },
  );

  html = html.replace(/`([^`]+)`/g, '<code class="inline-code">$1</code>');
  html = html.replace(/\*\*([^*]+)\*\*/g, "<strong>$1</strong>");
  html = html.replace(/\*([^*]+)\*/g, "<em>$1</em>");
  html = html.replace(/\n/g, "<br>");

  return html;
}
</script>

<template>
  <div class="flex flex-col h-[calc(100vh-7rem)] max-h-[calc(100vh-4rem)]">
    

    <!-- ========== 顶部: 消息区域 (flex-1 撑满) ========== -->
    <div
      ref="messagesContainer"
      class="overflow-y-auto px-4 py-4 space-y-4 min-h-0 scrollbar-hidden flex-1"
    >
      <!-- 空状态 -->
      <div
        v-if="messages.length === 0"
        class="flex flex-col items-center justify-center h-full text-muted-foreground gap-3"
      >
        <div class="flex items-center justify-center size-16 rounded-2xl bg-muted/50">
          <IconSparkles class="size-8" />
        </div>
        <p class="text-sm">Pick a model, and start a playground conversation</p>
        <p class="text-xs text-muted-foreground/60">
          Models are accessed from access group
        </p>
      </div>

      <!-- 消息气泡 -->
      <template v-for="(msg, i) in messages" :key="i" class="">
        <!-- 系统消息 -->
        <div v-if="msg.role === 'system'" class="flex justify-center">
          <div class="px-3 py-1.5 text-xs text-muted-foreground bg-muted/50 rounded-lg max-w-[80%] text-center">
            {{ msg.content }}
          </div>
        </div>

        <!-- 用户消息 -->
        <div v-else-if="msg.role === 'user'" class="flex justify-end">
          <div class="max-w-[80%] rounded-2xl rounded-br-md px-4 py-2.5 bg-primary text-primary-foreground text-sm">
            <div class="whitespace-pre-wrap wrap-break-word" v-text="msg.content"></div>
          </div>
        </div>

        <!-- 助手消息 -->
        <div v-else class="flex justify-start">
          <div class="max-w-[85%] rounded-2xl rounded-bl-md px-4 py-2.5 bg-muted text-sm">
            <!-- 等待中: 打字动画点 -->
            <div
              v-if="msg.content === '' && msg.thinkContent === '' && isStreaming"
              class="flex items-center gap-1 py-0.5"
            >
              <span class="size-1.5 rounded-full bg-muted-foreground/50 animate-bounce" style="animation-delay: 0ms"></span>
              <span class="size-1.5 rounded-full bg-muted-foreground/50 animate-bounce" style="animation-delay: 150ms"></span>
              <span class="size-1.5 rounded-full bg-muted-foreground/50 animate-bounce" style="animation-delay: 300ms"></span>
            </div>

            <!-- 思考内容（可折叠） -->
            <details v-if="msg.thinkContent" class="think-section mb-2" :open="isStreaming && i === messages.length - 1">
              <summary class="think-summary">
                <IconBrain class="size-3.5 shrink-0" />
                <span>Thinking</span>
              </summary>
              <div
                class="think-content prose prose-sm max-w-none [&_pre]:my-2 [&_pre]:p-3 [&_pre]:rounded-lg [&_pre]:bg-amber-100/30 dark:[&_pre]:bg-amber-900/20 [&_pre]:overflow-x-auto [&_code]:text-xs [&_.inline-code]:rounded [&_.inline-code]:bg-amber-100/50 dark:[&_.inline-code]:bg-amber-900/30 [&_.inline-code]:px-1 [&_.inline-code]:py-0.5 [&_.inline-code]:text-xs [&_.inline-code]:font-mono [&_.code-block]:font-mono [&_.code-block]:text-xs [&_strong]:font-semibold"
                v-html="renderContent(msg.thinkContent)"
              ></div>
            </details>

            <!-- 渲染 Markdown -->
            <div
              v-if="msg.content"
              class="prose prose-sm dark:prose-invert max-w-none wrap-break-word [&_pre]:my-2 [&_pre]:p-3 [&_pre]:rounded-lg [&_pre]:bg-muted-foreground/10 [&_pre]:overflow-x-auto [&_code]:text-xs [&_.inline-code]:rounded [&_.inline-code]:bg-muted-foreground/15 [&_.inline-code]:px-1 [&_.inline-code]:py-0.5 [&_.inline-code]:text-xs [&_.inline-code]:font-mono [&_.code-block]:font-mono [&_.code-block]:text-xs [&_strong]:font-semibold"
              v-html="renderContent(msg.content)"
            ></div>
            <!-- 流式光标 -->
            <span
              v-if="isStreaming && i === messages.length - 1 && (msg.content !== '' || msg.thinkContent !== '')"
              class="inline-block w-1.5 h-4 ml-1 -mb-0.5 bg-primary animate-pulse rounded-sm align-middle"
            ></span>
          </div>
        </div>
      </template>
    </div>
    <div class="bottom justify-end-safe flex flex-col ">
        <!-- ========== 中部: 模型选择 & 操作 ========== -->
        <div class="flex items-center justify-between shrink-0 px-4 py-3 border-t mt-3">
        <div class="flex items-center gap-3">
            <Select v-model="selectedModel" :disabled="isStreaming">
            <SelectTrigger class="w-56">
                <SelectValue placeholder="Pick a model...">
                <div v-if="selectedModel" class="flex items-center gap-2">
                    <IconBoxModel class="size-3.5 text-muted-foreground shrink-0" />
                    <span>{{ selectedModel }}</span>
                </div>
                </SelectValue>
            </SelectTrigger>
            <SelectContent>
                <SelectGroup>
                <SelectItem
                    v-for="model in modelOptions"
                    :key="model"
                    :value="model"
                >
                    <SelectItemText>{{ model }}</SelectItemText>
                </SelectItem>
                </SelectGroup>
            </SelectContent>
            </Select>

            <Badge
            v-if="selectedModel"
            variant="secondary"
            class="text-xs font-normal"
            >
            {{ selectedModel }}
            </Badge>
        </div>

        <Button
            variant="outline"
            size="sm"
            :disabled="messages.length === 0"
            @click="clearConversation"
        >
            <IconTrash class="size-4" />
            <span class="ml-1.5">Clear</span>
        </Button>
        </div>
        <!-- ========== 底部: 输入区域 ========== -->
        <div class="shrink-0 px-4 py-3">
            <div class="flex items-end gap-2 h-24">
                <Textarea
                v-model="inputText"
                :disabled="isStreaming || !selectedModel"
                :placeholder="selectedModel ? 'Enter your prompt... (Enter to send, and Shift+Enter for a new line)' : 'Please pick a model first :)'"
                class="flex-1 overflow-y-auto scrollbar-hidden h-24 resize-none"
                @keydown="handleKeydown"
                />
                <Button
                size="icon"
                class="shrink-0"
                :disabled="!inputText.trim() || !selectedModel || isStreaming"
                @click="sendMessage"
                >
                <IconSend class="size-4" />
                </Button>
            </div>
        </div>
    </div>

  </div>
</template>

<style scoped>
.think-section {
  border-radius: 0.5rem;
  overflow: hidden;
}

.think-summary {
  display: flex;
  align-items: center;
  gap: 0.375rem;
  padding: 0.375rem 0.5rem;
  font-size: 0.75rem;
  font-weight: 500;
  color: hsl(var(--muted-foreground));
  background: linear-gradient(135deg, hsl(45 20% 94%), hsl(40 15% 90%));
  border-radius: 0.375rem;
  cursor: pointer;
  user-select: none;
  transition: background 0.15s;
}

.dark .think-summary {
  background: linear-gradient(135deg, hsl(35 8% 18%), hsl(30 6% 14%));
}

.think-summary:hover {
  background: linear-gradient(135deg, hsl(45 30% 90%), hsl(40 25% 86%));
}

.dark .think-summary:hover {
  background: linear-gradient(135deg, hsl(35 10% 22%), hsl(30 8% 18%));
}

.think-summary::-webkit-details-marker {
  display: none;
}

.think-content {
  padding: 0.5rem 0.5rem 0.25rem;
  font-size: 0.8125rem;
  color: hsl(35 20% 30%);
  border-left: 2px solid hsl(40 30% 70%);
  margin-left: 0.25rem;
}

.dark .think-content {
  color: hsl(40 15% 70%);
  border-left-color: hsl(35 15% 35%);
}
</style>
