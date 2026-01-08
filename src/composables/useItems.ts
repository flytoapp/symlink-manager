import { ref, watch, type Ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { Item, SymlinkResult } from '@/types';

export function useItems(
  profileId: Ref<string | null>,
  sourceId: Ref<string | null>
) {
  const items = ref<Item[]>([]);
  const isLoading = ref(false);
  const isToggling = ref<Record<string, boolean>>({});
  const error = ref<string | null>(null);

  async function loadItems() {
    if (!profileId.value || !sourceId.value) {
      items.value = [];
      return;
    }

    isLoading.value = true;
    error.value = null;

    try {
      items.value = await invoke<Item[]>('get_items_with_status', {
        profileId: profileId.value,
        sourceId: sourceId.value,
      });
    } catch (e) {
      error.value = String(e);
      items.value = [];
    } finally {
      isLoading.value = false;
    }
  }

  async function toggleItem(
    itemName: string,
    enabled: boolean
  ): Promise<SymlinkResult> {
    if (!profileId.value || !sourceId.value) {
      return {
        success: false,
        itemName,
        error: 'No profile or source selected',
      };
    }

    isToggling.value[itemName] = true;

    try {
      const result = await invoke<SymlinkResult>('toggle_item', {
        profileId: profileId.value,
        sourceId: sourceId.value,
        itemName,
        enabled,
      });

      // Optimistically update the item in place
      if (result.success) {
        const item = items.value.find(i => i.name === itemName);
        if (item) {
          item.enabled = enabled;
          item.status = enabled ? 'active' : 'inactive';
        }
      }

      return result;
    } finally {
      isToggling.value[itemName] = false;
    }
  }

  // Auto-reload when profile or source changes
  watch([profileId, sourceId], loadItems, { immediate: true });

  return {
    items,
    isLoading,
    isToggling,
    error,
    loadItems,
    toggleItem,
  };
}
