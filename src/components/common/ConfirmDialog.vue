<script setup lang="ts">
defineProps<{
  show: boolean;
  title: string;
  message: string;
  confirmText?: string;
  cancelText?: string;
  danger?: boolean;
}>();

const emit = defineEmits<{
  confirm: [];
  cancel: [];
}>();
</script>

<template>
  <Teleport to="body">
    <div
      v-if="show"
      class="fixed inset-0 bg-black/50 flex items-center justify-center z-50"
      @click.self="emit('cancel')"
    >
      <div class="bg-white dark:bg-gray-800 rounded-lg p-6 max-w-md w-[90%] shadow-xl">
        <h3 class="text-lg font-semibold text-gray-900 dark:text-gray-100 mb-3">{{ title }}</h3>
        <p class="text-sm text-gray-600 dark:text-gray-400 mb-6">{{ message }}</p>
        <div class="flex gap-3 justify-end">
          <button
            class="px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-300 bg-gray-100 dark:bg-gray-700 rounded-md hover:bg-gray-200 dark:hover:bg-gray-600"
            @click="emit('cancel')"
          >
            {{ cancelText || 'Cancel' }}
          </button>
          <button
            class="px-4 py-2 text-sm font-medium text-white rounded-md"
            :class="danger ? 'bg-red-500 hover:bg-red-600' : 'bg-blue-500 hover:bg-blue-600'"
            @click="emit('confirm')"
          >
            {{ confirmText || 'Confirm' }}
          </button>
        </div>
      </div>
    </div>
  </Teleport>
</template>
