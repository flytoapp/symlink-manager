import { ref } from 'vue';

export type ThemeMode = 'light' | 'dark' | 'system';

const mode = ref<ThemeMode>((localStorage.getItem('theme') as ThemeMode) || 'system');

function applyTheme() {
  const isDark =
    mode.value === 'dark' ||
    (mode.value === 'system' && window.matchMedia('(prefers-color-scheme: dark)').matches);

  if (isDark) {
    document.documentElement.classList.add('dark');
  } else {
    document.documentElement.classList.remove('dark');
  }
}

function setMode(newMode: ThemeMode) {
  mode.value = newMode;
  localStorage.setItem('theme', newMode);
  applyTheme();
}

function cycleMode() {
  const modes: ThemeMode[] = ['light', 'dark', 'system'];
  const currentIndex = modes.indexOf(mode.value);
  const nextIndex = (currentIndex + 1) % modes.length;
  setMode(modes[nextIndex]);
}

// Listen for system preference changes
window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', () => {
  if (mode.value === 'system') {
    applyTheme();
  }
});

// Apply on load
applyTheme();

export function useTheme() {
  return {
    mode,
    setMode,
    cycleMode,
  };
}
