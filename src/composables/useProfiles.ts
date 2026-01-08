import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { Profile } from '@/types';
import { useAppStore } from '@/stores/appStore';

export function useProfiles() {
  const store = useAppStore();
  const isCreating = ref(false);
  const isUpdating = ref(false);
  const isDeleting = ref(false);

  async function createProfile(name: string, basePath: string): Promise<Profile> {
    isCreating.value = true;
    try {
      const profile = await invoke<Profile>('create_profile', { name, basePath });
      await store.reloadConfig();
      return profile;
    } finally {
      isCreating.value = false;
    }
  }

  async function updateProfile(profile: Profile): Promise<Profile> {
    isUpdating.value = true;
    try {
      const updated = await invoke<Profile>('update_profile', { profile });
      await store.reloadConfig();
      return updated;
    } finally {
      isUpdating.value = false;
    }
  }

  async function deleteProfile(profileId: string): Promise<void> {
    isDeleting.value = true;
    try {
      await invoke('delete_profile', { profileId });
      await store.reloadConfig();
      // If we deleted the active profile, clear the selection
      if (store.activeProfileId === profileId) {
        const remaining = store.profiles;
        store.setActiveProfile(remaining.length > 0 ? remaining[0].id : null);
      }
    } finally {
      isDeleting.value = false;
    }
  }

  return {
    profiles: () => store.profiles,
    activeProfile: () => store.activeProfile,
    isCreating,
    isUpdating,
    isDeleting,
    createProfile,
    updateProfile,
    deleteProfile,
    setActiveProfile: store.setActiveProfile,
  };
}
