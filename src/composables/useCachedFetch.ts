import { ref } from "vue";

const CACHE_PREFIX = "pontus_cache_";

function getTodayKey(): string {
  const now = new Date();
  return `${now.getFullYear()}-${now.getMonth() + 1}-${now.getDate()}`;
}

function cacheKey(name: string): string {
  return `${CACHE_PREFIX}${name}`;
}

function isCacheFresh(name: string): boolean {
  const stored = localStorage.getItem(`${cacheKey(name)}_date`);
  return stored === getTodayKey();
}

/**
 * Wraps an async fetch function with localStorage caching.
 * Data is cached per-day — re-fetches only when the date changes
 * or when `forceRefresh` is called.
 */
export function useCachedFetch<T>(name: string, fetcher: () => Promise<T>) {
  const data = ref<T | null>(null);
  const loading = ref(false);

  // Try load from cache
  function loadCached(): boolean {
    try {
      if (isCacheFresh(name)) {
        const raw = localStorage.getItem(cacheKey(name));
        if (raw) {
          data.value = JSON.parse(raw);
          return true;
        }
      }
    } catch {
      // corrupted cache — ignore
    }
    return false;
  }

  async function fetch(force = false): Promise<T> {
    if (!force && loadCached()) return data.value!;

    loading.value = true;
    try {
      const result = await fetcher();
      data.value = result;
      localStorage.setItem(cacheKey(name), JSON.stringify(result));
      localStorage.setItem(`${cacheKey(name)}_date`, getTodayKey());
      return result;
    } finally {
      loading.value = false;
    }
  }

  async function forceRefresh(): Promise<T> {
    return fetch(true);
  }

  return {
    data,
    loading,
    fetch,
    forceRefresh,
    isCached: () => isCacheFresh(name),
  };
}
