<script setup lang="ts">
import { ref, computed } from "vue";
import type { Group, Note } from "../composables/useApi";
import NoteItem from "./NoteItem.vue";
import { useSortable } from "@dnd-kit/vue/sortable";
import { DragDropProvider } from "@dnd-kit/vue";

const props = defineProps<{
  group: Group;
  index: number;
  notes: Note[];
}>();

const emit = defineEmits<{
  editGroup: [group: Group];
  deleteGroup: [group: Group];
  copyNote: [note: Note];
  editNote: [note: Note];
  deleteNote: [note: Note];
  reorderNotes: [groupId: number, event: any];
}>();

const expanded = ref(false);

const groupNotesModel = computed(() => {
  return props.notes;
});

function onNoteDragEnd(event: any) {
  if (!event.operation) return;
  emit("reorderNotes", props.group.id, event);
}

const element = ref<HTMLElement | null>(null);
const handle = ref<HTMLElement | null>(null);

const { isDragging } = useSortable({
  id: computed(() => props.group.id),
  index: computed(() => props.index),
  element,
  handle,
});

function toggle() {
  expanded.value = !expanded.value;
}
</script>

<template>
  <div ref="element" class="group-item" :class="{ 'is-dragging': isDragging }">
    <div class="group-header" @click="toggle">
      <!-- 拖拽手柄 -->
      <div
        ref="handle"
        class="group-drag-handle"
        title="拖动排序"
        @click.stop
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
      <div class="group-arrow" :class="{ open: expanded }">
        <svg width="8" height="8" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round">
          <polyline points="9 18 15 12 9 6" />
        </svg>
      </div>
      <span class="group-dot" />
      <span class="group-name">{{ group.name }}</span>
      <span class="group-count">{{ groupNotesModel.length }}</span>
      <div class="group-actions">
        <button class="act-btn" @click.stop="$emit('editGroup', group)" title="编辑" aria-label="编辑分组">
          <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7" />
            <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z" />
          </svg>
        </button>
        <button class="act-btn act-btn--danger" @click.stop="$emit('deleteGroup', group)" title="删除" aria-label="删除分组">
          <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <polyline points="3 6 5 6 21 6" />
            <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" />
          </svg>
        </button>
      </div>
    </div>

    <Transition name="slide">
      <div v-if="expanded" class="group-children">
        <template v-if="groupNotesModel.length > 0">
          <DragDropProvider @dragEnd="onNoteDragEnd">
            <NoteItem
              v-for="(note, noteIndex) in groupNotesModel"
              :key="note.id"
              :note="note"
              :index="noteIndex"
              @copy="$emit('copyNote', $event)"
              @edit="$emit('editNote', $event)"
              @delete="$emit('deleteNote', $event)"
            />
          </DragDropProvider>
        </template>
        <div v-else class="group-empty">暂无笔记</div>
      </div>
    </Transition>
  </div>
</template>

<style scoped>
.group-item {
}

.group-header {
  position: relative;
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 14px 4px 34px;
  cursor: pointer;
  user-select: none;
  transition: background var(--transition-fast);
}

.group-header:hover {
  background: rgba(0, 0, 0, 0.015);
}

.group-drag-handle {
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

.group-drag-handle:active {
  cursor: grabbing;
}

.group-header:hover .group-drag-handle {
  opacity: 0.5;
}

.group-drag-handle:hover {
  opacity: 1 !important;
  color: var(--color-text-secondary);
}

.group-arrow {
  width: 14px;
  height: 14px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--color-text-muted);
  transition: transform 0.2s ease;
  flex-shrink: 0;
}

.group-arrow.open {
  transform: rotate(90deg);
}

.group-dot {
  width: 7px;
  height: 7px;
  border-radius: 50%;
  flex-shrink: 0;
  background: #6366f1;
}

.group-name {
  font-size: 11px;
  font-weight: 600;
  color: var(--color-text-secondary);
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.group-count {
  font-size: var(--font-size-2xs);
  color: var(--color-text-muted);
  font-weight: 500;
  background: rgba(0, 0, 0, 0.03);
  padding: 1px 6px;
  border-radius: 8px;
  flex-shrink: 0;
  font-variant-numeric: tabular-nums;
}

.group-actions {
  display: flex;
  align-items: center;
  gap: 1px;
  opacity: 0;
  transition: opacity var(--transition-fast);
  flex-shrink: 0;
}

.group-header:hover .group-actions {
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
  min-width: 22px;
  min-height: 22px;
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

/* Children notes */
.group-children {
  overflow: hidden;
}

.group-empty {
  padding: 6px 14px 4px 34px;
  font-size: var(--font-size-xs);
  color: var(--color-text-muted);
}

.is-dragging {
  opacity: 0.4 !important;
  background: var(--color-surface-active) !important;
  box-shadow: var(--shadow-md);
  z-index: 10;
}

/* Slide transition */
.slide-enter-active {
  transition: all 180ms cubic-bezier(0.2, 0, 0, 1);
}
.slide-leave-active {
  transition: all 120ms ease;
}
.slide-enter-from,
.slide-leave-to {
  opacity: 0;
  transform: translateY(-4px);
}
</style>
