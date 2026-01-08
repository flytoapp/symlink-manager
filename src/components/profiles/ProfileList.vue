<script setup lang="ts">
import { ref } from 'vue';
import { Trash2 } from 'lucide-vue-next';
import type { Profile } from '@/types';
import ProfileForm from './ProfileForm.vue';
import ConfirmDialog from '@/components/common/ConfirmDialog.vue';
import { useProfiles } from '@/composables/useProfiles';

defineProps<{
  profiles: Profile[];
  activeProfileId: string | null;
}>();

const emit = defineEmits<{
  select: [profileId: string];
}>();

const { createProfile, deleteProfile, isCreating } = useProfiles();

const showCreateForm = ref(false);
const profileToDelete = ref<Profile | null>(null);

async function handleCreate(name: string, basePath: string) {
  const profile = await createProfile(name, basePath);
  showCreateForm.value = false;
  emit('select', profile.id);
}

async function handleDelete() {
  if (profileToDelete.value) {
    await deleteProfile(profileToDelete.value.id);
    profileToDelete.value = null;
  }
}
</script>

<template>
  <div class="flex flex-col gap-3">
    <div class="flex justify-between items-center">
      <h2 class="text-sm font-semibold text-gray-700 dark:text-gray-200">Profiles</h2>
      <button
        class="px-2.5 py-1.5 text-xs font-medium text-white bg-blue-500 rounded hover:bg-blue-600"
        @click="showCreateForm = true"
      >
        + New Profile
      </button>
    </div>

    <div v-if="profiles.length === 0" class="p-4 text-center text-sm text-gray-500 dark:text-gray-400">
      No profiles yet. Create one to get started.
    </div>

    <div v-else class="flex flex-col gap-2">
      <div
        v-for="profile in profiles"
        :key="profile.id"
        class="flex items-center gap-3 p-3 rounded-md cursor-pointer transition-colors border"
        :class="profile.id === activeProfileId
          ? 'bg-blue-50 dark:bg-blue-900/30 border-blue-500 dark:border-blue-400'
          : 'bg-gray-50 dark:bg-gray-700/50 border-gray-200 dark:border-gray-600 hover:bg-gray-100 dark:hover:bg-gray-700'"
        @click="emit('select', profile.id)"
      >
        <div class="flex-1 min-w-0 flex flex-col gap-0.5">
          <span class="text-sm font-medium text-gray-900 dark:text-gray-100">{{ profile.name }}</span>
          <span class="text-xs text-gray-500 dark:text-gray-400 truncate">{{ profile.basePath }}</span>
        </div>
        <button
          class="p-1.5 text-gray-400 dark:text-gray-500 hover:text-red-500 dark:hover:text-red-400 rounded hover:bg-red-50 dark:hover:bg-red-900/30 transition-colors"
          title="Delete profile"
          @click.stop="profileToDelete = profile"
        >
          <Trash2 :size="16" />
        </button>
      </div>
    </div>

    <ProfileForm
      v-if="showCreateForm"
      :is-loading="isCreating"
      @submit="handleCreate"
      @cancel="showCreateForm = false"
    />

    <ConfirmDialog
      :show="!!profileToDelete"
      title="Delete Profile"
      :message="`Are you sure you want to delete '${profileToDelete?.name}'? This will remove all symlinks managed by this profile.`"
      confirm-text="Delete"
      :danger="true"
      @confirm="handleDelete"
      @cancel="profileToDelete = null"
    />
  </div>
</template>
