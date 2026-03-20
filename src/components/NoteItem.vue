<script setup lang="ts">
import { ref, computed } from "vue";
import type { Note } from "../composables/useApi";
import { useSortable } from "@dnd-kit/vue/sortable";

const props = defineProps<{ 
  note: Note;
  index: number;
}>();

defineEmits<{
  copy: [note: Note];
  edit: [note: Note];
  delete: [note: Note];
}>();

function truncate(text: string, max: number) {
  if (text.length <= max) return text;
  return text.slice(0, max) + "...";
}

const element = ref<HTMLElement | null>(null);
const handle = ref<HTMLElement | null>(null);

const { isDragging } = useSortable({
  id: computed(() => props.note.id),
  index: computed(() => props.index),
  element,
  handle,
});
</script>

<template>
  <div ref="element" class="note-item" :class="{ 'is-dragging': isDragging }">
    <!-- 手柄作为选择器 -->
    <div
      ref="handle"
      class="drag-handle"
      title="拖动排序"
    >
      <svg style="pointer-events: none;" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
        <circle cx="9" cy="5" r="1" />
        <circle cx="9" cy="12" r="1" />
        <circle cx="9" cy="19" r="1" />
        <circle cx="15" cy="5" r="1" />
        <circle cx="15" cy="12" r="1" />
        <circle cx="15" cy="19" r="1" />
      </svg>
    </div>
    <div class="note-body" @click="$emit('copy', note)" :title="`点击复制: ${note.content}`">
      <span class="note-title">{{ note.name }}</span>
      <span class="note-preview">{{ truncate(note.content, 30) }}</span>
    </div>
    <div class="note-actions">
      <button class="act-btn" @click.stop="$emit('edit', note)" title="编辑" aria-label="编辑">
        <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7" />
          <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z" />
        </svg>
      </button>
      <button class="act-btn act-btn--danger" @click.stop="$emit('delete', note)" title="删除" aria-label="删除">
        <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <polyline points="3 6 5 6 21 6" />
          <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" />
        </svg>
      </button>
    </div>
  </div>
</template>

<style scoped>
.note-item {
  position: relative;
  display: flex;
  align-items: center;
  padding: 7px 14px 7px 34px;
  cursor: pointer;
  transition: background var(--transition-fast);
  user-select: none;
  background: var(--color-bg);
}

.is-dragging {
  opacity: 0.4 !important;
  background: var(--color-surface-active) !important;
  box-shadow: var(--shadow-md);
  z-index: 10;
}

.drag-handle {
  position: absolute;
  left: 0;
  top: 0;
  bottom: 0;
  width: 34px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--color-text-muted);
  cursor: grab;
  opacity: 0;
  transition: opacity var(--transition-fast);
}

.drag-handle:active {
  cursor: grabbing;
}

.note-item:hover .drag-handle {
  opacity: 0.5;
}

.drag-handle:hover {
  opacity: 1 !important;
  color: var(--color-text-secondary);
}

.note-item:hover {
  background: rgba(0, 0, 0, 0.025);
}

.note-body {
  flex: 1;
  display: flex;
  align-items: center;
  overflow: hidden;
  gap: 8px;
}

.note-title {
  flex: 1;
  font-size: var(--font-size-sm);
  font-weight: 500;
  color: var(--color-text-secondary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.note-preview {
  font-size: var(--font-size-xs);
  color: var(--color-text-muted);
  margin-left: 8px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  max-width: 100px;
  flex-shrink: 0;
}

.note-actions {
  display: flex;
  gap: 1px;
  opacity: 0;
  transition: opacity var(--transition-fast);
  flex-shrink: 0;
  margin-left: 8px;
}

.note-item:hover .note-actions {
  opacity: 1;
}

.note-item:focus-within .note-actions {
  opacity: 1;
}

.act-btn {
  background: none;
  border: none;
  padding: 4px;
  cursor: pointer;
  color: var(--color-text-muted);
  border-radius: var(--radius-xs);
  display: flex;
  align-items: center;
  justify-content: center;
  min-width: 24px;
  min-height: 24px;
  transition: background var(--transition-fast), color var(--transition-fast);
}

.act-btn:hover {
  background: var(--color-surface-active);
  color: var(--color-text-secondary);
}

.act-btn--danger:hover {
  background: var(--color-danger-subtle);
  color: var(--color-danger);
}
</style>
