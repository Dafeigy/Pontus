<script setup lang="ts">
import { computed } from 'vue'
import { useAppStore } from '@/stores/app'
import Button from '@/components/ui/button/Button.vue'
import { IconSun, IconMoon } from '@tabler/icons-vue'

const store = useAppStore()

const isDark = computed(() => store.config.theme === 'dark')

function toggleTheme(event: MouseEvent) {
  const x = event.clientX
  const y = event.clientY
  const switchingToDark = !isDark.value

  const endRadius = Math.hypot(
    Math.max(x, innerWidth - x),
    Math.max(y, innerHeight - y),
  )

  // Fallback for browsers without View Transitions API
  if (!document.startViewTransition) {
    const next = isDark.value ? 'light' : 'dark'
    store.config.theme = next
    store.applyTheme(next)
    return
  }

  // Light → Dark: new(root) on top, expand from click → dark spreads outward
  // Dark → Light: old(root) on top, shrink to click  → dark contracts inward
  const pseudo = switchingToDark
    ? '::view-transition-new(root)'
    : '::view-transition-old(root)'

  const styleEl = document.createElement('style')
  styleEl.textContent = `${pseudo} { z-index: 9999; }`
  document.head.appendChild(styleEl)

  const transition = document.startViewTransition(() => {
    const next = isDark.value ? 'light' : 'dark'
    store.config.theme = next
    store.applyTheme(next)
  })

  transition.ready.then(() => {
    const clipPath = switchingToDark
      ? [`circle(0px at ${x}px ${y}px)`, `circle(${endRadius}px at ${x}px ${y}px)`]
      : [`circle(${endRadius}px at ${x}px ${y}px)`, `circle(0px at ${x}px ${y}px)`]

    document.documentElement.animate(
      { clipPath },
      {
        duration: 400,
        easing: 'ease-in',
        fill: 'forwards',
        pseudoElement: pseudo,
      },
    )

    setTimeout(() => styleEl.remove(), 500)
  })
}
</script>

<template>
  <Button
    variant="ghost"
    size="icon"
    @click="toggleTheme"
  >
    <IconSun v-if="isDark" class="size-4.5" />
    <IconMoon v-else class="size-4.5" />
  </Button>
</template>
