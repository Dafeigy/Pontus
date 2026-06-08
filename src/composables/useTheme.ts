import { ref } from 'vue'

export type ThemeMode = 'auto' | 'light' | 'dark'

const themeMode = ref<ThemeMode>('auto')
const isDark = ref(false)

function applyTheme(mode: ThemeMode) {
  const root = document.documentElement

  if (mode === 'dark') {
    root.classList.add('dark')
    isDark.value = true
  } else if (mode === 'light') {
    root.classList.remove('dark')
    isDark.value = false
  } else {
    // auto: follow system preference
    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches
    if (prefersDark) {
      root.classList.add('dark')
      isDark.value = true
    } else {
      root.classList.remove('dark')
      isDark.value = false
    }
  }
  themeMode.value = mode
}

// Listen to system theme changes in auto mode
function setupAutoListener() {
  const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
  mediaQuery.addEventListener('change', (e) => {
    if (themeMode.value === 'auto') {
      if (e.matches) {
        document.documentElement.classList.add('dark')
        isDark.value = true
      } else {
        document.documentElement.classList.remove('dark')
        isDark.value = false
      }
    }
  })
}

// Cycle: auto → dark → light → auto
function cycleTheme(): ThemeMode {
  const cycle: ThemeMode[] = ['auto', 'dark', 'light']
  const idx = cycle.indexOf(themeMode.value)
  const next = cycle[(idx + 1) % cycle.length]
  applyTheme(next)
  return next
}

function getThemeLabel(mode: ThemeMode): string {
  switch (mode) {
    case 'auto': return 'Auto'
    case 'dark': return 'Dark'
    case 'light': return 'Light'
  }
}

// Initialize on import
setupAutoListener()

export function useTheme() {
  return {
    themeMode,
    isDark,
    applyTheme,
    cycleTheme,
    getThemeLabel,
  }
}
