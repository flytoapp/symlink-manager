<script setup lang="ts">
import { ref, watch, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { Folder, File, Check, Loader2 } from 'lucide-vue-next';
import type { Profile, Item, Source } from '@/types';

const props = defineProps<{
  profile: Profile;
}>();

interface EnabledItem extends Item {
  source: Source;
}

const allItems = ref<EnabledItem[]>([]);
const isLoading = ref(false);
const error = ref<string | null>(null);

async function loadAllItems() {
  if (!props.profile) return;

  isLoading.value = true;
  error.value = null;
  allItems.value = [];

  try {
    const results: EnabledItem[] = [];

    for (const source of props.profile.sources) {
      const items = await invoke<Item[]>('get_items_with_status', {
        profileId: props.profile.id,
        sourceId: source.id,
      });

      for (const item of items) {
        if (item.status === 'active') {
          results.push({ ...item, source });
        }
      }
    }

    // Sort by source name, then item name
    results.sort((a, b) => {
      const sourceCompare = a.source.name.localeCompare(b.source.name);
      if (sourceCompare !== 0) return sourceCompare;
      return a.name.localeCompare(b.name);
    });

    allItems.value = results;
  } catch (e) {
    error.value = String(e);
  } finally {
    isLoading.value = false;
  }
}

// Group items by source
const groupedItems = computed(() => {
  const groups = new Map<string, { source: Source; items: EnabledItem[] }>();

  for (const item of allItems.value) {
    const key = item.source.id;
    if (!groups.has(key)) {
      groups.set(key, { source: item.source, items: [] });
    }
    groups.get(key)!.items.push(item);
  }

  return Array.from(groups.values());
});

watch(() => props.profile.id, loadAllItems, { immediate: true });
</script>

<template>
  <div class="flex flex-col h-full">
    <div class="mb-4">
      <h2 class="text-xl font-semibold text-gray-900 dark:text-gray-100">{{ profile.name }}</h2>
      <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">
        {{ allItems.length }} active item{{ allItems.length === 1 ? '' : 's' }} across {{ profile.sources.length }} source{{ profile.sources.length === 1 ? '' : 's' }}
      </p>
    </div>

    <div v-if="isLoading" class="flex-1 flex items-center justify-center text-gray-500 dark:text-gray-400">
      <Loader2 :size="20" class="animate-spin mr-2" />
      Loading items...
    </div>

    <div v-else-if="error" class="flex-1 flex items-center justify-center text-red-500 dark:text-red-400">
      {{ error }}
    </div>

    <div v-else-if="allItems.length === 0" class="flex-1 flex flex-col items-center justify-center text-gray-500 dark:text-gray-400">
      <p>No active symlinks in this profile.</p>
      <p class="text-sm mt-2">Select a source from the sidebar to enable items.</p>
    </div>

    <div v-else class="flex-1 overflow-y-auto space-y-4">
      <div
        v-for="group in groupedItems"
        :key="group.source.id"
        class="bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 overflow-hidden"
      >
        <div class="px-3 py-2 bg-gray-50 dark:bg-gray-700/50 border-b border-gray-200 dark:border-gray-700">
          <h3 class="text-sm font-medium text-gray-700 dark:text-gray-300">{{ group.source.name }}</h3>
          <p class="text-xs text-gray-500 dark:text-gray-400 truncate">{{ group.source.sourcePath }}</p>
        </div>
        <div class="divide-y divide-gray-100 dark:divide-gray-700">
          <div
            v-for="item in group.items"
            :key="item.name"
            class="flex items-center gap-2 px-3 py-2 text-sm"
          >
            <component
              :is="item.isDirectory ? Folder : File"
              :size="14"
              class="flex-shrink-0 text-emerald-500 dark:text-emerald-400"
            />
            <span class="flex-1 truncate text-gray-700 dark:text-gray-300">{{ item.name }}</span>
            <Check :size="14" class="flex-shrink-0 text-emerald-500 dark:text-emerald-400" />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
