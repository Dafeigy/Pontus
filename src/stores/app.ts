import { defineStore } from "pinia";
import { ref, watch } from "vue";

export interface AppConfig {
  apiKey: string;
  litellmHost: string;
  smtpHost: string;
  smtpPort: number;
  smtpSenderEmail: string;
  smtpUsername: string;
  smtpPassword: string;
  theme: "light" | "dark";
}

export const useAppStore = defineStore("app", () => {
  const isInitialized = ref(false);
  const config = ref<AppConfig>({
    apiKey: "",
    litellmHost: "http://localhost:4000",
    smtpHost: "",
    smtpPort: 587,
    smtpSenderEmail: "",
    smtpUsername: "",
    smtpPassword: "",
    theme: "light",
  });

  function applyTheme(theme: "light" | "dark") {
    const root = document.documentElement;
    if (theme === "dark") {
      root.classList.add("dark");
    } else {
      root.classList.remove("dark");
    }
  }

  watch(
    () => config.value.theme,
    (newTheme) => applyTheme(newTheme),
    { immediate: true }
  );

  return {
    isInitialized,
    config,
    applyTheme,
  };
});
