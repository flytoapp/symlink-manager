<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { Folder, File, Loader2, Check, FolderOpen } from 'lucide-vue-next';
import { revealItemInDir } from '@tauri-apps/plugin-opener';
import { platform } from '@tauri-apps/plugin-os';
import type { Item } from '@/types';

const fileManagerName = ref('File Manager');

onMounted(async () => {
  const os = await platform();
  if (os === 'macos') {
    fileManagerName.value = 'Finder';
  } else if (os === 'windows') {
    fileManagerName.value = 'Explorer';
  } else {
    fileManagerName.value = 'Files';
  }
});

const props = defineProps<{
  item: Item;
  isToggling: boolean;
  disabled: boolean;
}>();

const emit = defineEmits<{
  toggle: [itemName: string, enabled: boolean];
}>();

const showContextMenu = ref(false);
const contextMenuPos = ref({ x: 0, y: 0 });

// Close this menu when another context menu opens
function closeMenu() {
  showContextMenu.value = false;
}

onMounted(() => {
  window.addEventListener('contextmenu-opened', closeMenu);
});

onUnmounted(() => {
  window.removeEventListener('contextmenu-opened', closeMenu);
});

function handleClick() {
  if (!props.disabled && !props.isToggling && props.item.status !== 'conflict') {
    // Toggle based on current status: if active, disable; otherwise enable
    emit('toggle', props.item.name, props.item.status !== 'active');
  }
}

function handleContextMenu(e: MouseEvent) {
  e.preventDefault();

  // Close any other open context menus
  window.dispatchEvent(new CustomEvent('contextmenu-opened'));

  contextMenuPos.value = { x: e.clientX, y: e.clientY };
  showContextMenu.value = true;

  // Close menu when clicking elsewhere
  const handleClose = () => {
    showContextMenu.value = false;
    document.removeEventListener('click', handleClose);
  };
  setTimeout(() => document.addEventListener('click', handleClose), 0);
}

async function openInFinder() {
  showContextMenu.value = false;
  try {
    await revealItemInDir(props.item.sourcePath);
  } catch (e) {
    console.error('Failed to open path:', e);
  }
}
</script>

<template>
  <div
    class="flex items-center gap-2 px-2.5 py-1.5 rounded transition-colors cursor-pointer select-none"
    :class="{
      'opacity-50 cursor-not-allowed': disabled || item.status === 'conflict',
      'bg-emerald-100 dark:bg-emerald-900/40 text-emerald-800 dark:text-emerald-300 hover:bg-emerald-200 dark:hover:bg-emerald-900/60': item.status === 'active',
      'bg-gray-50 dark:bg-gray-800 text-gray-500 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700': item.status === 'inactive' || item.status === 'broken',
      'bg-red-100 dark:bg-red-900/40 text-red-700 dark:text-red-300 cursor-not-allowed': item.status === 'conflict',
    }"
    :title="item.status === 'conflict' ? `Conflict: ${item.conflictSource}` : (item.status === 'active' ? 'Click to disable' : 'Click to enable')"
    @click="handleClick"
    @contextmenu="handleContextMenu"
  >
    <Loader2 v-if="isToggling" :size="16" class="flex-shrink-0 animate-spin text-gray-400" />
    <component
      v-else
      :is="item.isDirectory ? Folder : File"
      :size="16"
      class="flex-shrink-0"
    />

    <span class="flex-1 min-w-0 text-[13px] truncate" :title="item.name">
      {{ item.name }}
    </span>

    <Check v-if="item.status === 'active' && !isToggling" :size="16" class="flex-shrink-0 text-emerald-500 dark:text-emerald-400" />
  </div>

  <!-- Context Menu -->
  <Teleport to="body">
    <div
      v-if="showContextMenu"
      class="fixed z-50 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg shadow-lg py-1 min-w-[160px]"
      :style="{ left: contextMenuPos.x + 'px', top: contextMenuPos.y + 'px' }"
    >
      <button
        class="w-full flex items-center gap-2 px-3 py-1.5 text-sm text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer"
        @click="openInFinder"
      >
        <FolderOpen :size="14" />
        Reveal in {{ fileManagerName }}
      </button>
    </div>
  </Teleport>
</template>
