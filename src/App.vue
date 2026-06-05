<script setup lang="ts">
import { onMounted, ref, computed } from "vue";
import { useRoute } from "vue-router";
import { useAppStore } from "@/stores/app";
import { invoke } from "@tauri-apps/api/core";
import AppSidebar from "@/components/AppSidebar.vue";
import SiteHeader from "@/components/SiteHeader.vue";
import InitialSetup from "@/components/InitialSetup.vue";
import { Sonner } from "@/components/ui/sonner";
import { SidebarInset, SidebarProvider } from "@/components/ui/sidebar";
import type { AppConfig } from "@/stores/app";
import { IconUserPlus, IconLayoutDashboard, IconSettings } from "@tabler/icons-vue";

const store = useAppStore();
const route = useRoute();
const loading = ref(true);

const data = {
  user: {
    name: "Admin",
    email: "admin@pontus.local",
    avatar: "",
  },
  navMain: [
    {
      title: "Invite",
      url: "#",
      icon: IconUserPlus,
      path: "/invite",
    },
    {
      title: "Dashboard",
      url: "#",
      icon: IconLayoutDashboard,
      path: "/dashboard",
    },
  ],
  navSecondary: [
    {
      title: "Settings",
      url: "#",
      icon: IconSettings,
      path: "/settings",
    },
  ],
};

onMounted(async () => {
  try {
    const initialized: boolean = await invoke("is_initialized_cmd");
    store.isInitialized = initialized;

    if (initialized) {
      const config: AppConfig = await invoke("get_config_cmd");
      store.config = config;
      store.applyTheme(config.theme as "light" | "dark");
    }
  } catch (e) {
    console.error("Failed to load config:", e);
  } finally {
    loading.value = false;
  }
});

function onSetupComplete(config: AppConfig) {
  store.config = config;
  store.isInitialized = true;
  store.applyTheme(config.theme as "light" | "dark");
}

function isActive(path: string) {
  return route.path === path || (path === "/invite" && route.path === "/");
}

const activeTitle = computed(() => {
  const allNav = [...data.navMain, ...data.navSecondary];
  const active = allNav.find((item) => isActive(item.path));
  return active?.title ?? "Pontus";
});
</script>

<template>
  <div v-if="loading" class="flex h-screen items-center justify-center bg-background">
    <p class="text-muted-foreground">正在加载...</p>
  </div>
  <template v-else>
    <InitialSetup v-if="!store.isInitialized" @complete="onSetupComplete" />
    <SidebarProvider
      v-else
      :style="{
        '--sidebar-width': 'calc(var(--spacing) * 72)',
        '--header-height': 'calc(var(--spacing) * 12)',
      }"
    >
      <AppSidebar variant="inset" :data="data" :isActive="isActive" />
      <SidebarInset>
        <SiteHeader :siteHeader="activeTitle" />
        <div class="flex flex-1 flex-col">
          <div class="@container/main flex flex-1 flex-col gap-2">
            <div class="flex flex-col gap-4 py-4 md:gap-6 md:py-6">
              <div class="px-4 lg:px-6">
                <main class="flex-1 overflow-auto">
                  <router-view />
                </main>
              </div>
            </div>
          </div>
        </div>
      </SidebarInset>
    </SidebarProvider>
    <Sonner />
  </template>
</template>
