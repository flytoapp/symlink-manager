<script setup lang="ts">
import { Sun, Moon, Monitor, ArrowLeft, RefreshCw, Download, CheckCircle, AlertCircle } from 'lucide-vue-next';
import { useTheme, type ThemeMode } from '@/composables/useTheme';
import { useUpdater } from '@/composables/useUpdater';

const emit = defineEmits<{
  back: [];
}>();

const { mode, setMode } = useTheme();
const { status, error, updateInfo, downloadProgress, checkForUpdates, downloadAndInstall } = useUpdater();

const themeOptions: { value: ThemeMode; label: string; icon: typeof Sun }[] = [
  { value: 'light', label: 'Light', icon: Sun },
  { value: 'dark', label: 'Dark', icon: Moon },
  { value: 'system', label: 'System', icon: Monitor },
];

const appVersion = __APP_VERSION__;
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
      <div class="max-w-2xl mx-auto space-y-6">
        <!-- Appearance Section -->
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

        <!-- Updates Section -->
        <section class="bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 p-4">
          <h2 class="text-sm font-semibold text-gray-900 dark:text-gray-100 mb-4">Updates</h2>

          <div class="flex flex-col gap-4">
            <!-- Current Version -->
            <div class="flex items-center justify-between">
              <span class="text-sm text-gray-600 dark:text-gray-400">Current version</span>
              <span class="text-sm font-medium text-gray-900 dark:text-gray-100">v{{ appVersion }}</span>
            </div>

            <!-- Status Display -->
            <div v-if="status === 'up-to-date'" class="flex items-center gap-2 text-green-600 dark:text-green-400">
              <CheckCircle :size="18" />
              <span class="text-sm">You're up to date!</span>
            </div>

            <div v-else-if="status === 'error'" class="flex items-center gap-2 text-red-600 dark:text-red-400">
              <AlertCircle :size="18" />
              <span class="text-sm">{{ error }}</span>
            </div>

            <div v-else-if="status === 'available' && updateInfo" class="bg-blue-50 dark:bg-blue-900/30 rounded-lg p-3">
              <div class="flex items-center justify-between mb-2">
                <span class="text-sm font-medium text-blue-700 dark:text-blue-300">
                  Version {{ updateInfo.version }} available
                </span>
              </div>
              <p v-if="updateInfo.body" class="text-sm text-blue-600 dark:text-blue-400 mb-3">
                {{ updateInfo.body }}
              </p>
              <button
                class="w-full flex items-center justify-center gap-2 px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors"
                @click="downloadAndInstall"
              >
                <Download :size="18" />
                <span>Download and Install</span>
              </button>
            </div>

            <div v-else-if="status === 'downloading' || status === 'installing'" class="space-y-2">
              <div class="flex items-center justify-between text-sm">
                <span class="text-gray-600 dark:text-gray-400">
                  {{ status === 'downloading' ? 'Downloading...' : 'Installing...' }}
                </span>
                <span class="text-gray-900 dark:text-gray-100">{{ downloadProgress }}%</span>
              </div>
              <div class="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
                <div
                  class="bg-blue-600 h-2 rounded-full transition-all duration-300"
                  :style="{ width: `${downloadProgress}%` }"
                />
              </div>
            </div>

            <!-- Check Button -->
            <button
              v-if="status === 'idle' || status === 'checking' || status === 'up-to-date' || status === 'error'"
              class="flex items-center justify-center gap-2 px-4 py-2 bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 text-gray-700 dark:text-gray-300 rounded-lg transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
              :disabled="status === 'checking'"
              @click="checkForUpdates"
            >
              <RefreshCw :size="18" :class="{ 'animate-spin': status === 'checking' }" />
              <span>{{ status === 'checking' ? 'Checking...' : 'Check for Updates' }}</span>
            </button>
          </div>
        </section>
      </div>
    </main>
  </div>
</template>
