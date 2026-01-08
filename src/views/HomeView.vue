<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { Settings, ExternalLink } from 'lucide-vue-next';
import { revealItemInDir } from '@tauri-apps/plugin-opener';
import { useAppStore } from '@/stores/appStore';
import ProfileList from '@/components/profiles/ProfileList.vue';
import ProfileSummary from '@/components/profiles/ProfileSummary.vue';
import SourceList from '@/components/sources/SourceList.vue';
import ItemList from '@/components/items/ItemList.vue';

const emit = defineEmits<{
  openSettings: [];
}>();

const store = useAppStore();

async function openInFinder(path: string) {
  try {
    await revealItemInDir(path);
  } catch (e) {
    console.error('Failed to open path:', e);
  }
}

const activeSourceId = ref<string | null>(null);

const activeProfile = computed(() => store.activeProfile);

const activeSource = computed(() => {
  if (!activeProfile.value || !activeSourceId.value) return null;
  return activeProfile.value.sources.find((s) => s.id === activeSourceId.value);
});

watch(
  () => store.activeProfileId,
  () => {
    activeSourceId.value = null;
  }
);

function selectProfile(profileId: string) {
  store.setActiveProfile(profileId);
  activeSourceId.value = null;
}

function selectSource(sourceId: string) {
  activeSourceId.value = sourceId;
}
</script>

<template>
  <div class="flex h-screen bg-gray-100 dark:bg-gray-900">
    <aside class="w-80 bg-white dark:bg-gray-800 border-r border-gray-200 dark:border-gray-700 flex flex-col overflow-y-auto">
      <div class="p-4 border-b border-gray-200 dark:border-gray-700">
        <ProfileList
          :profiles="store.profiles"
          :active-profile-id="store.activeProfileId"
          @select="selectProfile"
        />
      </div>

      <div v-if="activeProfile" class="p-4 flex-1">
        <SourceList
          :sources="activeProfile.sources"
          :profile-id="activeProfile.id"
          :profile-base-path="activeProfile.basePath"
          :active-source-id="activeSourceId"
          @select="selectSource"
        />
      </div>

      <div class="p-4 border-t border-gray-200 dark:border-gray-700 mt-auto">
        <button
          class="flex items-center gap-2 px-3 py-2 w-full text-sm text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-md transition-colors"
          @click="emit('openSettings')"
        >
          <Settings :size="16" />
          <span>Settings</span>
        </button>
      </div>
    </aside>

    <main class="flex-1 flex flex-col overflow-hidden">
      <div v-if="!activeProfile" class="flex-1 flex flex-col items-center justify-center p-8 text-center text-gray-500 dark:text-gray-400">
        <h2 class="text-xl font-semibold text-gray-700 dark:text-gray-200 mb-2">Welcome to Symlink Manager</h2>
        <p>Create a profile to get started managing your symlinks.</p>
      </div>

      <div v-else-if="!activeSource" class="flex-1 flex flex-col p-4 overflow-hidden">
        <ProfileSummary :profile="activeProfile" />
      </div>

      <div v-else class="flex-1 flex flex-col p-4 overflow-hidden">
        <div class="mb-4">
          <h2 class="text-lg font-semibold text-gray-900 dark:text-gray-100">{{ activeSource.name }}</h2>
          <p class="text-xs text-gray-500 dark:text-gray-400 mt-1 flex items-center gap-1">
            Source:
            <button
              class="inline-flex items-center gap-1 px-1 py-0.5 bg-gray-200 dark:bg-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600 rounded font-mono transition-colors cursor-pointer"
              title="Open in file manager"
              @click="openInFinder(activeSource.sourcePath)"
            >
              {{ activeSource.sourcePath }}
              <ExternalLink :size="10" class="opacity-50" />
            </button>
          </p>
          <p class="text-xs text-gray-500 dark:text-gray-400 flex items-center gap-1">
            Target:
            <button
              class="inline-flex items-center gap-1 px-1 py-0.5 bg-gray-200 dark:bg-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600 rounded font-mono transition-colors cursor-pointer"
              title="Open in file manager"
              @click="openInFinder(activeSource.targetPath || activeProfile.basePath)"
            >
              {{ activeSource.targetPath || activeProfile.basePath }}
              <ExternalLink :size="10" class="opacity-50" />
            </button>
          </p>
        </div>

        <ItemList
          :profile-id="activeProfile.id"
          :source-id="activeSource.id"
          :can-create-symlinks="store.canCreateSymlinks"
        />
      </div>
    </main>
  </div>
</template>
