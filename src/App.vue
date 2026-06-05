<script setup lang="ts">
import AppSidebar from "@/components/AppSidebar.vue"
import SiteHeader from "@/components/SiteHeader.vue"
import { useRoute } from "vue-router";
import { computed } from "vue";
import {
  SidebarInset,
  SidebarProvider,
} from "@/components/ui/sidebar"

import {
  IconDashboard,
  IconListDetails,
  IconSettings,
} from "@tabler/icons-vue"

const data = {
  user: {
    name: "noreply",
    email: "noreply@example.com",
    avatar: "/avatars/shadcn.jpg",
  },
  navMain: [
    {
      title: "Home",
      url: "#",
      icon: IconDashboard,
      path: "/home"
    },
    {
      title: "Dashboard",
      url: "#",
      icon: IconListDetails,
      path: "/dashboard"
    }
  ],
  navSecondary: [
    {
      title: "Settings",
      url: "#",
      icon: IconSettings,
      path: "/settings"
    }
  ]
}
const route = useRoute();

function isActive(path: string) {
  return route.path === path || (path === "/home" && route.path === "/");
}

const activeTitle = computed(() => {
  const allNav = [...data.navMain, ...data.navSecondary]
  const active = allNav.find(item => isActive(item.path))
  return active?.title ?? "Home"
})
</script>

<template>
  <SidebarProvider
    :style=" {
      '--sidebar-width': 'calc(var(--spacing) * 72)',
      '--header-height': 'calc(var(--spacing) * 12)',
    }"
  >
    <AppSidebar variant="inset" :data="data" :isActive="isActive"/>
    <SidebarInset>
      <SiteHeader :siteHeader="activeTitle"/>
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
</template>