<script setup lang="ts">
import { ref, computed } from 'vue';
import PathInput from '@/components/common/PathInput.vue';

const props = defineProps<{
  isLoading?: boolean;
  defaultTarget: string;
}>();

const emit = defineEmits<{
  submit: [name: string, sourcePath: string, targetPath?: string];
  cancel: [];
}>();

const name = ref('');
const sourcePath = ref('');
const useCustomTarget = ref(false);
const customTargetPath = ref('');

const resolvedTarget = computed(() => {
  return useCustomTarget.value && customTargetPath.value ? customTargetPath.value : props.defaultTarget;
});

function handleSubmit() {
  if (name.value.trim() && sourcePath.value.trim()) {
    const targetPath =
      useCustomTarget.value && customTargetPath.value.trim() ? customTargetPath.value.trim() : undefined;
    emit('submit', name.value.trim(), sourcePath.value.trim(), targetPath);
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
      <h3 class="text-lg font-semibold text-gray-900 dark:text-gray-100">Add Source</h3>

      <div class="flex flex-col gap-1.5">
        <label class="text-sm font-medium text-gray-700 dark:text-gray-300">Source Name</label>
        <input
          v-model="name"
          type="text"
          class="px-3 py-2 text-sm border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
          placeholder="e.g., Texture Mods"
          required
        />
      </div>

      <PathInput
        v-model="sourcePath"
        label="Source Folder"
        placeholder="Folder containing items to manage"
      />

      <div class="flex flex-col gap-3 p-3 bg-gray-50 dark:bg-gray-700/50 rounded-md">
        <label class="flex items-center gap-2 text-sm text-gray-700 dark:text-gray-300 cursor-pointer">
          <input v-model="useCustomTarget" type="checkbox" class="rounded w-4 h-4 border-2 border-gray-400 dark:border-gray-500 bg-transparent dark:bg-gray-600 checked:bg-blue-500 dark:checked:bg-blue-500" />
          <span>Use custom target folder</span>
        </label>

        <PathInput
          v-if="useCustomTarget"
          v-model="customTargetPath"
          label="Custom Target Folder"
          placeholder="Where symlinks will be created"
          :default-path="defaultTarget"
        />

        <div class="flex flex-col gap-1">
          <span class="text-xs text-gray-500 dark:text-gray-400">Symlinks will be created in:</span>
          <code class="text-xs text-gray-800 dark:text-gray-200 break-all">{{ resolvedTarget }}</code>
        </div>
      </div>

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
          class="px-4 py-2 text-sm font-medium text-white bg-emerald-500 rounded-md hover:bg-emerald-600 disabled:opacity-50 disabled:cursor-not-allowed"
          :disabled="isLoading || !name.trim() || !sourcePath.trim()"
        >
          {{ isLoading ? 'Adding...' : 'Add Source' }}
        </button>
      </div>
    </form>
  </div>
</template>
