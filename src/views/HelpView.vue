<script setup lang="ts">
import { computed } from 'vue';
import { ArrowLeft } from 'lucide-vue-next';
import { marked, Renderer } from 'marked';
import { openUrl } from '@tauri-apps/plugin-opener';
import manualContent from '../../docs/MANUAL.md?raw';

const emit = defineEmits<{
  back: [];
}>();

// Custom renderer to add IDs to headings for TOC links
const renderer = new Renderer();
renderer.heading = ({ text, depth }) => {
  const slug = text
    .toLowerCase()
    .replace(/<[^>]*>/g, '') // Strip HTML tags
    .replace(/[^\w\s-]/g, '') // Remove special chars
    .replace(/\s+/g, '-'); // Spaces to hyphens
  return `<h${depth} id="${slug}">${text}</h${depth}>`;
};

const htmlContent = computed(() => marked(manualContent, { renderer }));

// Handle link clicks - open external URLs in system browser
function handleClick(e: MouseEvent) {
  const target = e.target as HTMLElement;
  const link = target.closest('a');
  if (!link) return;

  const href = link.getAttribute('href');
  // Only open in external browser if it's a full URL, not an anchor link
  if (href?.startsWith('http')) {
    e.preventDefault();
    openUrl(href);
  }
}
</script>

<template>
  <div class="h-screen bg-gray-100 dark:bg-gray-900 flex flex-col">
    <header class="bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700 px-4 py-3">
      <div class="flex items-center gap-3">
        <button
          class="p-1.5 text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-md transition-colors"
          @click="emit('back')"
        >
          <ArrowLeft :size="20" />
        </button>
        <h1 class="text-lg font-semibold text-gray-900 dark:text-gray-100">Help</h1>
      </div>
    </header>

    <main class="flex-1 overflow-y-auto p-6">
      <div class="max-w-3xl mx-auto">
        <article class="prose dark:prose-invert prose-gray max-w-none" v-html="htmlContent" @click="handleClick" />
      </div>
    </main>
  </div>
</template>

<style>
.prose {
  color: var(--tw-prose-body);
  line-height: 1.75;
}

.prose :where(h1):not(:where([class~="not-prose"] *)) {
  font-size: 2em;
  font-weight: 700;
  margin-top: 0;
  margin-bottom: 0.75em;
  line-height: 1.2;
}

.prose :where(h2):not(:where([class~="not-prose"] *)) {
  font-size: 1.5em;
  font-weight: 600;
  margin-top: 1.75em;
  margin-bottom: 0.75em;
  line-height: 1.3;
  padding-bottom: 0.3em;
  border-bottom: 1px solid;
  border-color: rgb(229 231 235);
}

.dark .prose :where(h2):not(:where([class~="not-prose"] *)) {
  border-color: rgb(55 65 81);
}

.prose :where(h3):not(:where([class~="not-prose"] *)) {
  font-size: 1.25em;
  font-weight: 600;
  margin-top: 1.5em;
  margin-bottom: 0.5em;
  line-height: 1.4;
}

.prose :where(p):not(:where([class~="not-prose"] *)) {
  margin-top: 1em;
  margin-bottom: 1em;
}

.prose :where(a):not(:where([class~="not-prose"] *)) {
  color: rgb(59 130 246);
  text-decoration: underline;
}

.prose :where(a):not(:where([class~="not-prose"] *)):hover {
  color: rgb(37 99 235);
}

.prose :where(strong):not(:where([class~="not-prose"] *)) {
  font-weight: 600;
}

.prose :where(ul):not(:where([class~="not-prose"] *)) {
  list-style-type: disc;
  margin-top: 1em;
  margin-bottom: 1em;
  padding-left: 1.5em;
}

.prose :where(ol):not(:where([class~="not-prose"] *)) {
  list-style-type: decimal;
  margin-top: 1em;
  margin-bottom: 1em;
  padding-left: 1.5em;
}

.prose :where(li):not(:where([class~="not-prose"] *)) {
  margin-top: 0.25em;
  margin-bottom: 0.25em;
}

.prose :where(blockquote):not(:where([class~="not-prose"] *)) {
  border-left: 4px solid rgb(229 231 235);
  padding-left: 1em;
  margin-top: 1em;
  margin-bottom: 1em;
  font-style: italic;
  color: rgb(107 114 128);
}

.dark .prose :where(blockquote):not(:where([class~="not-prose"] *)) {
  border-color: rgb(75 85 99);
  color: rgb(156 163 175);
}

.prose :where(code):not(:where([class~="not-prose"] *)) {
  background-color: rgb(243 244 246);
  padding: 0.2em 0.4em;
  border-radius: 0.25em;
  font-size: 0.875em;
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
}

.dark .prose :where(code):not(:where([class~="not-prose"] *)) {
  background-color: rgb(55 65 81);
}

.prose :where(pre):not(:where([class~="not-prose"] *)) {
  background-color: rgb(31 41 55);
  color: rgb(229 231 235);
  padding: 1em;
  border-radius: 0.5em;
  overflow-x: auto;
  margin-top: 1em;
  margin-bottom: 1em;
}

.prose :where(pre code):not(:where([class~="not-prose"] *)) {
  background-color: transparent;
  padding: 0;
  font-size: 0.875em;
}

.prose :where(table):not(:where([class~="not-prose"] *)) {
  width: 100%;
  border-collapse: collapse;
  margin-top: 1em;
  margin-bottom: 1em;
  font-size: 0.875em;
}

.prose :where(th):not(:where([class~="not-prose"] *)) {
  background-color: rgb(243 244 246);
  padding: 0.5em 1em;
  text-align: left;
  font-weight: 600;
  border: 1px solid rgb(229 231 235);
}

.dark .prose :where(th):not(:where([class~="not-prose"] *)) {
  background-color: rgb(55 65 81);
  border-color: rgb(75 85 99);
}

.prose :where(td):not(:where([class~="not-prose"] *)) {
  padding: 0.5em 1em;
  border: 1px solid rgb(229 231 235);
}

.dark .prose :where(td):not(:where([class~="not-prose"] *)) {
  border-color: rgb(75 85 99);
}

.prose :where(hr):not(:where([class~="not-prose"] *)) {
  border: 0;
  border-top: 1px solid rgb(229 231 235);
  margin-top: 2em;
  margin-bottom: 2em;
}

.dark .prose :where(hr):not(:where([class~="not-prose"] *)) {
  border-color: rgb(55 65 81);
}

.prose :where(img):not(:where([class~="not-prose"] *)) {
  max-width: 100%;
  height: auto;
  border-radius: 0.5em;
  margin-top: 1em;
  margin-bottom: 1em;
}

/* Dark mode text colors */
.dark .prose {
  color: rgb(209 213 219);
}

.prose :where(h1, h2, h3, h4, h5, h6):not(:where([class~="not-prose"] *)) {
  color: rgb(17 24 39);
}

.dark .prose :where(h1, h2, h3, h4, h5, h6):not(:where([class~="not-prose"] *)) {
  color: rgb(243 244 246);
}
</style>
