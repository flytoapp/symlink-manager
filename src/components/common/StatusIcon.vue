<script setup lang="ts">
import { computed } from 'vue';
import { Check, Circle, AlertTriangle } from 'lucide-vue-next';
import type { ItemStatus } from '@/types';

const props = defineProps<{
  status: ItemStatus;
  conflictSource?: string;
}>();

const statusConfig = computed(() => {
  switch (props.status) {
    case 'active':
      return { icon: Check, color: 'text-emerald-500', tooltip: 'Symlink active' };
    case 'inactive':
      return { icon: Circle, color: 'text-gray-400', tooltip: 'Not linked' };
    case 'broken':
      return { icon: AlertTriangle, color: 'text-amber-500', tooltip: 'Broken symlink' };
    case 'conflict':
      return {
        icon: AlertTriangle,
        color: 'text-red-500',
        tooltip: props.conflictSource ? `Conflict: ${props.conflictSource}` : 'Name conflict',
      };
  }
});
</script>

<template>
  <span class="cursor-help select-none flex items-center" :class="statusConfig.color" :title="statusConfig.tooltip">
    <component :is="statusConfig.icon" :size="16" :stroke-width="2.5" />
  </span>
</template>
