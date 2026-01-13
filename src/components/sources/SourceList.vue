<script setup lang="ts">
import { ref } from 'vue';
import { Trash2, Folder, ArrowRight, Pencil } from 'lucide-vue-next';
import type { Source } from '@/types';
import SourceForm from './SourceForm.vue';
import ConfirmDialog from '@/components/common/ConfirmDialog.vue';
import { useSources } from '@/composables/useSources';

const props = defineProps<{
  sources: Source[];
  profileId: string;
  profileBasePath: string;
  activeSourceId: string | null;
}>();

const emit = defineEmits<{
  select: [sourceId: string];
}>();

const { createSource, updateSource, deleteSource, isCreating, isUpdating } = useSources();

const showCreateForm = ref(false);
const sourceToEdit = ref<Source | null>(null);
const sourceToDelete = ref<Source | null>(null);

async function handleCreate(name: string, sourcePath: string, targetPath?: string) {
  const source = await createSource(props.profileId, name, sourcePath, targetPath);
  showCreateForm.value = false;
  emit('select', source.id);
}

async function handleEdit(name: string, sourcePath: string, targetPath?: string) {
  if (!sourceToEdit.value) return;
  await updateSource(props.profileId, {
    ...sourceToEdit.value,
    name,
    sourcePath,
    targetPath,
  });
  sourceToEdit.value = null;
}

async function handleDelete() {
  if (sourceToDelete.value) {
    await deleteSource(props.profileId, sourceToDelete.value.id);
    sourceToDelete.value = null;
  }
}

function getResolvedTarget(source: Source): string {
  return source.targetPath || props.profileBasePath;
}
</script>

<template>
  <div class="flex flex-col gap-3">
    <div class="flex justify-between items-center">
      <h3 class="text-sm font-semibold text-gray-700 dark:text-gray-200">Sources</h3>
      <button
        class="px-2 py-1 text-xs font-medium text-white bg-emerald-500 rounded hover:bg-emerald-600"
        @click="showCreateForm = true"
      >
        + Add Source
      </button>
    </div>

    <div v-if="sources.length === 0" class="p-3 text-center text-sm text-gray-500 dark:text-gray-400 bg-gray-50 dark:bg-gray-700/50 rounded-md">
      No sources added yet.
    </div>

    <div v-else class="flex flex-col gap-1.5">
      <div
        v-for="source in sources"
        :key="source.id"
        class="flex items-start gap-2 p-2.5 rounded-md cursor-pointer transition-colors border"
        :class="source.id === activeSourceId
          ? 'bg-emerald-50 dark:bg-emerald-900/30 border-emerald-500 dark:border-emerald-400'
          : 'bg-gray-50 dark:bg-gray-700/50 border-gray-200 dark:border-gray-600 hover:bg-gray-100 dark:hover:bg-gray-700'"
        @click="emit('select', source.id)"
      >
        <div class="flex-1 min-w-0 flex flex-col gap-0.5">
          <span class="text-[13px] font-medium text-gray-900 dark:text-gray-100">{{ source.name }}</span>
          <span class="flex items-center gap-1 text-[11px] text-gray-500 dark:text-gray-400 truncate" title="Source folder">
            <Folder :size="11" class="flex-shrink-0" />
            {{ source.sourcePath }}
          </span>
          <span class="flex items-center gap-1 text-[11px] text-gray-400 dark:text-gray-500 truncate" title="Target folder">
            <ArrowRight :size="11" class="flex-shrink-0" />
            {{ getResolvedTarget(source) }}
          </span>
        </div>
        <div class="flex gap-0.5">
          <button
            class="p-1 text-gray-400 dark:text-gray-500 hover:text-blue-500 dark:hover:text-blue-400 rounded hover:bg-blue-50 dark:hover:bg-blue-900/30 transition-colors"
            title="Edit source"
            @click.stop="sourceToEdit = source"
          >
            <Pencil :size="14" />
          </button>
          <button
            class="p-1 text-gray-400 dark:text-gray-500 hover:text-red-500 dark:hover:text-red-400 rounded hover:bg-red-50 dark:hover:bg-red-900/30 transition-colors"
            title="Delete source"
            @click.stop="sourceToDelete = source"
          >
            <Trash2 :size="14" />
          </button>
        </div>
      </div>
    </div>

    <SourceForm
      v-if="showCreateForm"
      :is-loading="isCreating"
      :default-target="profileBasePath"
      @submit="handleCreate"
      @cancel="showCreateForm = false"
    />

    <SourceForm
      v-if="sourceToEdit"
      :source="sourceToEdit"
      :is-loading="isUpdating"
      :default-target="profileBasePath"
      @submit="handleEdit"
      @cancel="sourceToEdit = null"
    />

    <ConfirmDialog
      :show="!!sourceToDelete"
      title="Delete Source"
      :message="`Are you sure you want to delete '${sourceToDelete?.name}'? This will remove all symlinks from this source.`"
      confirm-text="Delete"
      :danger="true"
      @confirm="handleDelete"
      @cancel="sourceToDelete = null"
    />
  </div>
</template>
