<script setup lang="ts">
import { Sun, Moon, Monitor, ArrowLeft } from 'lucide-vue-next';
import { useTheme, type ThemeMode } from '@/composables/useTheme';

const emit = defineEmits<{
  back: [];
}>();

const { mode, setMode } = useTheme();

const themeOptions: { value: ThemeMode; label: string; icon: typeof Sun }[] = [
  { value: 'light', label: 'Light', icon: Sun },
  { value: 'dark', label: 'Dark', icon: Moon },
  { value: 'system', label: 'System', icon: Monitor },
];
</script>

<template>
  <div class="h-screen bg-gray-100 dark:bg-gray-900 flex flex-col">
    <header class="bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700 px-4 py-3">
      <div class="flex items-center gap-3">
        <button
          class="p-1.5 text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-md transition-colors"
          @click="emit('back')"
        >
          <ArrowLeft :size="20" />
        </button>
        <h1 class="text-lg font-semibold text-gray-900 dark:text-gray-100">Settings</h1>
      </div>
    </header>

    <main class="flex-1 overflow-y-auto p-6">
      <div class="max-w-2xl mx-auto">
        <section class="bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 p-4">
          <h2 class="text-sm font-semibold text-gray-900 dark:text-gray-100 mb-4">Appearance</h2>

          <div class="flex flex-col gap-2">
            <label class="text-sm text-gray-600 dark:text-gray-400 mb-1">Theme</label>
            <div class="flex gap-2">
              <button
                v-for="option in themeOptions"
                :key="option.value"
                class="flex-1 flex items-center justify-center gap-2 px-4 py-3 rounded-lg border-2 transition-colors"
                :class="mode === option.value
                  ? 'border-blue-500 bg-blue-50 dark:bg-blue-900/30 text-blue-700 dark:text-blue-300'
                  : 'border-gray-200 dark:border-gray-600 bg-gray-50 dark:bg-gray-700/50 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700'"
                @click="setMode(option.value)"
              >
                <component :is="option.icon" :size="18" />
                <span class="text-sm font-medium">{{ option.label }}</span>
              </button>
            </div>
          </div>
        </section>
      </div>
    </main>
  </div>
</template>
