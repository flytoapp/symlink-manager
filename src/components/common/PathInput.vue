<script setup lang="ts">
import { ref, watch } from 'vue';
import { open } from '@tauri-apps/plugin-dialog';

const props = defineProps<{
  modelValue: string;
  label: string;
  placeholder?: string;
  defaultPath?: string;
}>();

const emit = defineEmits<{
  'update:modelValue': [value: string];
}>();

const localValue = ref(props.modelValue);

watch(
  () => props.modelValue,
  (val) => {
    localValue.value = val;
  }
);

async function browseFolder() {
  const selected = await open({
    directory: true,
    multiple: false,
    title: `Select ${props.label}`,
    defaultPath: props.defaultPath,
  });

  if (selected && typeof selected === 'string') {
    localValue.value = selected;
    emit('update:modelValue', selected);
  }
}

function onInput(event: Event) {
  const value = (event.target as HTMLInputElement).value;
  localValue.value = value;
  emit('update:modelValue', value);
}
</script>

<template>
  <div class="flex flex-col gap-1.5">
    <label class="text-sm font-medium text-gray-700 dark:text-gray-300">{{ label }}</label>
    <div class="flex gap-2">
      <input
        type="text"
        class="flex-1 px-3 py-2 text-sm border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
        :value="localValue"
        :placeholder="placeholder"
        @input="onInput"
      />
      <button
        type="button"
        class="px-3 py-2 text-sm bg-gray-100 dark:bg-gray-700 border border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 rounded-md hover:bg-gray-200 dark:hover:bg-gray-600 whitespace-nowrap"
        @click="browseFolder"
      >
        Browse...
      </button>
    </div>
  </div>
</template>
