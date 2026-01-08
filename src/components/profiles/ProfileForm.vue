<script setup lang="ts">
import { ref } from 'vue';
import PathInput from '@/components/common/PathInput.vue';

defineProps<{
  isLoading?: boolean;
}>();

const emit = defineEmits<{
  submit: [name: string, basePath: string];
  cancel: [];
}>();

const name = ref('');
const basePath = ref('');

function handleSubmit() {
  if (name.value.trim() && basePath.value.trim()) {
    emit('submit', name.value.trim(), basePath.value.trim());
  }
}
</script>

<template>
  <div
    class="fixed inset-0 bg-black/50 flex items-center justify-center z-50"
    @click.self="emit('cancel')"
  >
    <form
      class="bg-white dark:bg-gray-800 rounded-lg p-6 max-w-lg w-[90%] flex flex-col gap-4"
      @submit.prevent="handleSubmit"
    >
      <h3 class="text-lg font-semibold text-gray-900 dark:text-gray-100">New Profile</h3>

      <div class="flex flex-col gap-1.5">
        <label class="text-sm font-medium text-gray-700 dark:text-gray-300">Profile Name</label>
        <input
          v-model="name"
          type="text"
          class="px-3 py-2 text-sm border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
          placeholder="e.g., My Game Mods"
          required
        />
      </div>

      <PathInput
        v-model="basePath"
        label="Default Target Folder"
        placeholder="Where symlinks will be created by default"
      />

      <p class="text-xs text-gray-500 dark:text-gray-400">
        This is where symlinks will be created unless a source specifies its own target.
      </p>

      <div class="flex gap-3 justify-end mt-2">
        <button
          type="button"
          class="px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-300 bg-gray-100 dark:bg-gray-700 rounded-md hover:bg-gray-200 dark:hover:bg-gray-600"
          @click="emit('cancel')"
        >
          Cancel
        </button>
        <button
          type="submit"
          class="px-4 py-2 text-sm font-medium text-white bg-blue-500 rounded-md hover:bg-blue-600 disabled:opacity-50 disabled:cursor-not-allowed"
          :disabled="isLoading || !name.trim() || !basePath.trim()"
        >
          {{ isLoading ? 'Creating...' : 'Create Profile' }}
        </button>
      </div>
    </form>
  </div>
</template>
