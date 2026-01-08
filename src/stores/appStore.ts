import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { AppConfig, PermissionStatus } from '@/types';
import { invoke } from '@tauri-apps/api/core';

export const useAppStore = defineStore('app', () => {
  // State
  const config = ref<AppConfig | null>(null);
  const activeProfileId = ref<string | null>(null);
  const permissionStatus = ref<PermissionStatus | null>(null);
  const isLoading = ref(false);
  const error = ref<string | null>(null);

  // Getters
  const profiles = computed(() => config.value?.profiles ?? []);

  const activeProfile = computed(() =>
    profiles.value.find((p) => p.id === activeProfileId.value) ?? null
  );

  const canCreateSymlinks = computed(
    () => permissionStatus.value?.canCreateSymlinks ?? true
  );

  // Actions
  async function initialize() {
    isLoading.value = true;
    error.value = null;

    try {
      const [loadedConfig, permissions] = await Promise.all([
        invoke<AppConfig>('load_config'),
        invoke<PermissionStatus>('check_symlink_permissions'),
      ]);

      config.value = loadedConfig;
      permissionStatus.value = permissions;

      if (loadedConfig.activeProfileId) {
        activeProfileId.value = loadedConfig.activeProfileId;
      } else if (loadedConfig.profiles.length > 0) {
        activeProfileId.value = loadedConfig.profiles[0].id;
      }
    } catch (e) {
      error.value = String(e);
    } finally {
      isLoading.value = false;
    }
  }

  async function setActiveProfile(profileId: string | null) {
    activeProfileId.value = profileId;
    try {
      await invoke('set_active_profile', { profileId });
    } catch (e) {
      console.error('Failed to save active profile:', e);
    }
  }

  function updateConfig(newConfig: AppConfig) {
    config.value = newConfig;
  }

  async function reloadConfig() {
    try {
      const loadedConfig = await invoke<AppConfig>('load_config');
      config.value = loadedConfig;
    } catch (e) {
      error.value = String(e);
    }
  }

  return {
    config,
    activeProfileId,
    permissionStatus,
    isLoading,
    error,
    profiles,
    activeProfile,
    canCreateSymlinks,
    initialize,
    setActiveProfile,
    updateConfig,
    reloadConfig,
  };
});
