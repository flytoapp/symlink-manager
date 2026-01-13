import { ref, readonly } from 'vue';
import { check, type Update } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';

export type UpdateStatus =
  | 'idle'
  | 'checking'
  | 'available'
  | 'downloading'
  | 'installing'
  | 'up-to-date'
  | 'error';

export interface UpdateInfo {
  version: string;
  date?: string;
  body?: string;
}

const status = ref<UpdateStatus>('idle');
const error = ref<string | null>(null);
const updateInfo = ref<UpdateInfo | null>(null);
const downloadProgress = ref<number>(0);

let currentUpdate: Update | null = null;

async function checkForUpdates(): Promise<boolean> {
  status.value = 'checking';
  error.value = null;
  updateInfo.value = null;
  downloadProgress.value = 0;

  try {
    const update = await check();

    if (update) {
      currentUpdate = update;
      updateInfo.value = {
        version: update.version,
        date: update.date,
        body: update.body ?? undefined,
      };
      status.value = 'available';
      return true;
    } else {
      status.value = 'up-to-date';
      return false;
    }
  } catch (e) {
    error.value = e instanceof Error ? e.message : String(e);
    status.value = 'error';
    return false;
  }
}

async function downloadAndInstall(): Promise<void> {
  if (!currentUpdate) {
    error.value = 'No update available to install';
    status.value = 'error';
    return;
  }

  status.value = 'downloading';
  error.value = null;

  try {
    let totalLength = 0;
    let downloadedLength = 0;

    await currentUpdate.downloadAndInstall((event) => {
      switch (event.event) {
        case 'Started':
          totalLength = event.data.contentLength ?? 0;
          downloadedLength = 0;
          downloadProgress.value = 0;
          break;
        case 'Progress':
          downloadedLength += event.data.chunkLength;
          if (totalLength > 0) {
            downloadProgress.value = Math.round((downloadedLength / totalLength) * 100);
          }
          break;
        case 'Finished':
          downloadProgress.value = 100;
          status.value = 'installing';
          break;
      }
    });

    await relaunch();
  } catch (e) {
    error.value = e instanceof Error ? e.message : String(e);
    status.value = 'error';
  }
}

function reset(): void {
  status.value = 'idle';
  error.value = null;
  updateInfo.value = null;
  downloadProgress.value = 0;
  currentUpdate = null;
}

export function useUpdater() {
  return {
    status: readonly(status),
    error: readonly(error),
    updateInfo: readonly(updateInfo),
    downloadProgress: readonly(downloadProgress),
    checkForUpdates,
    downloadAndInstall,
    reset,
  };
}
