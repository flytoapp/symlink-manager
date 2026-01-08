<script setup lang="ts">
import { computed, ref } from 'vue';
import { RefreshCw, X, Search, XCircle } from 'lucide-vue-next';
import ItemRow from './ItemRow.vue';
import { useItems } from '@/composables/useItems';

const props = defineProps<{
  profileId: string;
  sourceId: string;
  canCreateSymlinks: boolean;
}>();

const profileIdRef = computed(() => props.profileId);
const sourceIdRef = computed(() => props.sourceId);

const { items, isLoading, isToggling, error, toggleItem, loadItems } = useItems(profileIdRef, sourceIdRef);

const lastError = ref<string | null>(null);
const filter = ref<'all' | 'enabled' | 'disabled'>('all');
const searchQuery = ref('');

function fuzzyMatch(text: string, query: string): boolean {
  const textLower = text.toLowerCase();
  const queryLower = query.toLowerCase();
  let textIndex = 0;

  for (const char of queryLower) {
    const foundIndex = textLower.indexOf(char, textIndex);
    if (foundIndex === -1) return false;
    textIndex = foundIndex + 1;
  }

  return true;
}

const filteredItems = computed(() => {
  let result = items.value;

  // Apply status filter
  if (filter.value === 'enabled') result = result.filter(i => i.enabled);
  else if (filter.value === 'disabled') result = result.filter(i => !i.enabled);

  // Apply fuzzy search filter
  if (searchQuery.value.trim()) {
    result = result.filter(i => fuzzyMatch(i.name, searchQuery.value));
  }

  return result;
});

const counts = computed(() => ({
  all: items.value.length,
  enabled: items.value.filter(i => i.enabled).length,
  disabled: items.value.filter(i => !i.enabled).length,
}));

async function handleToggle(itemName: string, enabled: boolean) {
  lastError.value = null;
  const result = await toggleItem(itemName, enabled);
  if (!result.success && result.error) {
    lastError.value = result.error;
  }
}

const isDisablingAll = ref(false);

async function disableAll() {
  const enabledItems = items.value.filter(i => i.enabled && i.status !== 'conflict');
  if (enabledItems.length === 0) return;

  isDisablingAll.value = true;
  lastError.value = null;

  for (const item of enabledItems) {
    const result = await toggleItem(item.name, false);
    if (!result.success && result.error) {
      lastError.value = result.error;
      break;
    }
  }

  isDisablingAll.value = false;
}
</script>

<template>
  <div class="flex flex-col gap-3 h-full">
    <div class="flex justify-between items-center">
      <h3 class="text-sm font-semibold text-gray-700 dark:text-gray-200">Items</h3>
      <div class="flex gap-2">
        <button
          v-if="counts.enabled > 0"
          class="flex items-center gap-1.5 px-2 py-1 text-xs bg-red-50 dark:bg-red-900/30 border border-red-200 dark:border-red-800 text-red-600 dark:text-red-400 rounded hover:bg-red-100 dark:hover:bg-red-900/50 disabled:opacity-50"
          :disabled="isLoading || isDisablingAll"
          @click="disableAll"
        >
          <XCircle :size="12" />
          {{ isDisablingAll ? 'Disabling...' : 'Disable All' }}
        </button>
        <button
          class="flex items-center gap-1.5 px-2 py-1 text-xs bg-gray-100 dark:bg-gray-700 border border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 rounded hover:bg-gray-200 dark:hover:bg-gray-600 disabled:opacity-50"
          :disabled="isLoading"
          @click="loadItems"
        >
          <RefreshCw :size="12" />
          Refresh
        </button>
      </div>
    </div>

    <div class="flex gap-1 p-1 bg-gray-100 dark:bg-gray-800 rounded-lg">
      <button
        v-for="f in (['all', 'enabled', 'disabled'] as const)"
        :key="f"
        class="flex-1 px-3 py-1.5 text-xs font-medium rounded-md transition-colors"
        :class="filter === f
          ? 'bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 shadow-sm'
          : 'text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-gray-200'"
        @click="filter = f"
      >
        {{ f === 'all' ? 'All' : f === 'enabled' ? 'Enabled' : 'Disabled' }}
        <span class="ml-1 text-gray-400 dark:text-gray-500">({{ counts[f] }})</span>
      </button>
    </div>

    <div class="relative">
      <Search :size="14" class="absolute left-2.5 top-1/2 -translate-y-1/2 text-gray-400 dark:text-gray-500" />
      <input
        v-model="searchQuery"
        type="text"
        placeholder="Search items..."
        class="w-full pl-8 pr-8 py-1.5 text-sm border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 placeholder-gray-400 dark:placeholder-gray-500 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
      />
      <button
        v-if="searchQuery"
        class="absolute right-2 top-1/2 -translate-y-1/2 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300"
        @click="searchQuery = ''"
      >
        <X :size="14" />
      </button>
    </div>

    <div v-if="lastError" class="p-3 bg-red-50 dark:bg-red-900/30 border border-red-200 dark:border-red-800 rounded-md text-sm text-red-600 dark:text-red-400 flex justify-between items-start gap-2">
      <span class="whitespace-pre-wrap">{{ lastError }}</span>
      <button class="text-red-600 dark:text-red-400 hover:text-red-800 dark:hover:text-red-300 p-0.5" @click="lastError = null">
        <X :size="16" />
      </button>
    </div>

    <div v-if="!canCreateSymlinks" class="p-3 bg-amber-50 dark:bg-amber-900/30 border border-amber-200 dark:border-amber-800 rounded-md text-sm text-amber-800 dark:text-amber-300">
      <strong class="block mb-1">⚠ Permission Required</strong>
      <p class="m-0">
        Symlink creation requires additional permissions on Windows. Enable Developer Mode in Settings or run as Administrator.
      </p>
    </div>

    <div v-if="isLoading" class="flex-1 flex items-center justify-center text-gray-500 dark:text-gray-400">
      Loading items...
    </div>

    <div v-else-if="error" class="flex-1 flex items-center justify-center text-red-500 dark:text-red-400">
      {{ error }}
    </div>

    <div v-else-if="items.length === 0" class="flex-1 flex items-center justify-center text-gray-500 dark:text-gray-400">
      No items found in this source folder.
    </div>

    <div v-else-if="filteredItems.length === 0" class="flex-1 flex items-center justify-center text-gray-500 dark:text-gray-400">
      <span v-if="searchQuery">No items matching "{{ searchQuery }}"</span>
      <span v-else>No {{ filter === 'enabled' ? 'enabled' : 'disabled' }} items.</span>
    </div>

    <div v-else class="flex-1 overflow-y-auto flex flex-col gap-1 pr-1">
      <ItemRow
        v-for="item in filteredItems"
        :key="item.name"
        :item="item"
        :is-toggling="isToggling[item.name] || false"
        :disabled="!canCreateSymlinks"
        @toggle="handleToggle"
      />
    </div>

    <div class="flex gap-4 pt-2 border-t border-gray-200 dark:border-gray-700 text-[11px] text-gray-500 dark:text-gray-400">
      <span class="flex items-center gap-1.5"><span class="w-2.5 h-2.5 rounded-sm bg-emerald-200 dark:bg-emerald-700"></span> Enabled</span>
      <span class="flex items-center gap-1.5"><span class="w-2.5 h-2.5 rounded-sm bg-gray-200 dark:bg-gray-600"></span> Disabled</span>
      <span class="flex items-center gap-1.5"><span class="w-2.5 h-2.5 rounded-sm bg-red-200 dark:bg-red-700"></span> Conflict</span>
    </div>
  </div>
</template>
