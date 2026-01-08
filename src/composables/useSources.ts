import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { Source } from '@/types';
import { useAppStore } from '@/stores/appStore';

export function useSources() {
  const store = useAppStore();
  const isCreating = ref(false);
  const isUpdating = ref(false);
  const isDeleting = ref(false);

  async function createSource(
    profileId: string,
    name: string,
    sourcePath: string,
    targetPath?: string
  ): Promise<Source> {
    isCreating.value = true;
    try {
      const source = await invoke<Source>('create_source', {
        profileId,
        name,
        sourcePath,
        targetPath: targetPath || null,
      });
      await store.reloadConfig();
      return source;
    } finally {
      isCreating.value = false;
    }
  }

  async function updateSource(profileId: string, source: Source): Promise<Source> {
    isUpdating.value = true;
    try {
      const updated = await invoke<Source>('update_source', { profileId, source });
      await store.reloadConfig();
      return updated;
    } finally {
      isUpdating.value = false;
    }
  }

  async function deleteSource(profileId: string, sourceId: string): Promise<void> {
    isDeleting.value = true;
    try {
      await invoke('delete_source', { profileId, sourceId });
      await store.reloadConfig();
    } finally {
      isDeleting.value = false;
    }
  }

  return {
    isCreating,
    isUpdating,
    isDeleting,
    createSource,
    updateSource,
    deleteSource,
  };
}
