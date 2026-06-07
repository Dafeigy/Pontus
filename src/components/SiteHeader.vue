<script setup lang="ts">
import { Separator } from '@/components/ui/separator'
import { SidebarTrigger } from '@/components/ui/sidebar'
import { Button } from '@/components/ui/button'
import { useAppStore } from '@/stores/app'
import { IconSun, IconMoon } from '@tabler/icons-vue'
import { computed } from 'vue'

defineProps(['siteHeader'])

const store = useAppStore()

const isDark = computed(() => store.config.theme === 'dark')

function toggleTheme() {
  const next = isDark.value ? 'light' : 'dark'
  store.config.theme = next
  store.applyTheme(next)
}
</script>

<template>
  <header class="flex h-(--header-height) shrink-0 items-center gap-2 border-b transition-[width,height] ease-linear group-has-data-[collapsible=icon]/sidebar-wrapper:h-(--header-height)">
    <div class="flex w-full items-center gap-1 px-4 lg:gap-2 lg:px-6">
      <SidebarTrigger class="-ml-1" />
      <Separator
        orientation="vertical"
        class="mx-2 data-[orientation=vertical]:h-4"
      />
      <h1 class="text-base font-medium">
        {{ siteHeader }}
      </h1>
      <div class="ml-auto flex items-center">
        <Button variant="ghost" size="icon" @click="toggleTheme">
          <IconSun v-if="isDark" class="size-4.5" />
          <IconMoon v-else class="size-4.5" />
        </Button>
      </div>
    </div>
  </header>
</template>
