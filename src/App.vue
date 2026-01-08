<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { useAppStore } from '@/stores/appStore';
import HomeView from '@/views/HomeView.vue';
import SettingsView from '@/views/SettingsView.vue';
import '@/composables/useTheme'; // Initialize theme on load

const store = useAppStore();
const currentView = ref<'home' | 'settings'>('home');

onMounted(() => {
  store.initialize();
});
</script>

<template>
  <div v-if="store.isLoading" class="h-screen flex flex-col items-center justify-center text-gray-500 dark:text-gray-400 bg-gray-100 dark:bg-gray-900">
    <p>Loading...</p>
  </div>

  <div v-else-if="store.error" class="h-screen flex flex-col items-center justify-center p-8 text-center bg-gray-100 dark:bg-gray-900">
    <h2 class="text-xl font-semibold text-red-600 dark:text-red-400 mb-2">Error</h2>
    <p class="text-gray-600 dark:text-gray-400 mb-4">{{ store.error }}</p>
    <button
      class="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600"
      @click="store.initialize()"
    >
      Retry
    </button>
  </div>

  <SettingsView v-else-if="currentView === 'settings'" @back="currentView = 'home'" />

  <HomeView v-else @open-settings="currentView = 'settings'" />
</template>
