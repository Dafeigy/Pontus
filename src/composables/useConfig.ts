import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface Config {
  api_key: string | null
  smtp_host: string
  smtp_port: number
  smtp_sender_email: string
  smtp_username: string
  smtp_password: string
  litellm_host: string
  theme: string
}

const defaultConfig: Config = {
  api_key: null,
  smtp_host: '',
  smtp_port: 465,
  smtp_sender_email: '',
  smtp_username: '',
  smtp_password: '',
  litellm_host: '',
  theme: 'auto',
}

const config = ref<Config>({ ...defaultConfig })
const loading = ref(true)
const error = ref<string | null>(null)

export function useConfig() {
  async function load() {
    loading.value = true
    error.value = null
    try {
      config.value = await invoke<Config>('load_config')
    } catch (e) {
      error.value = String(e)
      config.value = { ...defaultConfig }
    } finally {
      loading.value = false
    }
  }

  async function save(): Promise<void> {
    error.value = null
    try {
      await invoke('save_config', { newConfig: config.value })
    } catch (e) {
      error.value = String(e)
      throw e
    }
  }

  async function resetApiKey(): Promise<void> {
    error.value = null
    try {
      await invoke('reset_api_key')
      config.value.api_key = null
    } catch (e) {
      error.value = String(e)
      throw e
    }
  }

  function isFirstRun(): boolean {
    return config.value.api_key === null || config.value.api_key === ''
  }

  onMounted(() => {
    load()
  })

  return {
    config,
    loading,
    error,
    load,
    save,
    resetApiKey,
    isFirstRun,
  }
}
