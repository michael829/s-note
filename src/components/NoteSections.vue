<script setup lang="ts">
import { DragDropProvider } from "@dnd-kit/vue";
import type { Group, Note } from "../composables/useApi";
import NoteItem from "./NoteItem.vue";
import GroupItem from "./GroupItem.vue";
import ErrorBanner from "./ErrorBanner.vue";

defineProps<{
  groups: Group[];
  ungroupedNotes: Note[];
  notesByGroup: Record<number, Note[]>;
  loading: boolean;
  error: string | null;
}>();

defineEmits<{
  copy: [note: Note];
  editNote: [note: Note];
  deleteNote: [note: Note];
  editGroup: [group: Group];
  deleteGroup: [group: Group];
  reorderUngrouped: [event: any];
  reorderGroups: [event: any];
  reorderGroupNotes: [groupId: number, event: any];
}>();
</script>

<template>
  <div class="list-area">
    <ErrorBanner :message="error" />
    <div v-if="!error && loading" class="state-banner">加载中...</div>

    <template v-if="ungroupedNotes.length === 0 && groups.length === 0 && !loading">
      <div class="empty-state">
        <div class="empty-icon-wrap">
          <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1" stroke-linecap="round" stroke-linejoin="round">
            <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
            <polyline points="14 2 14 8 20 8" />
            <line x1="16" y1="13" x2="8" y2="13" />
            <line x1="16" y1="17" x2="8" y2="17" />
          </svg>
        </div>
        <span class="empty-text">暂无笔记</span>
        <span class="empty-hint">在上方输入开始记录</span>
      </div>
    </template>

    <template v-if="ungroupedNotes.length > 0">
      <div class="section-header">
        <span class="section-label">笔记</span>
        <span class="section-badge">{{ ungroupedNotes.length }}</span>
      </div>
      <DragDropProvider @dragEnd="$emit('reorderUngrouped', $event)">
        <div class="dnd-list">
          <NoteItem
            v-for="(note, noteIndex) in ungroupedNotes"
            :key="note.id"
            :note="note"
            :index="noteIndex"
            @copy="$emit('copy', $event)"
            @edit="$emit('editNote', $event)"
            @delete="$emit('deleteNote', $event)"
          />
        </div>
      </DragDropProvider>
    </template>

    <template v-if="groups.length > 0">
      <div class="section-header" :class="{ 'section-header--spaced': ungroupedNotes.length > 0 }">
        <span class="section-label">分组</span>
        <span class="section-badge">{{ groups.length }}</span>
      </div>
      <DragDropProvider @dragEnd="$emit('reorderGroups', $event)">
        <div class="dnd-list">
          <GroupItem
            v-for="(group, groupIndex) in groups"
            :key="group.id"
            :group="group"
            :index="groupIndex"
            :notes="notesByGroup[group.id] ?? []"
            @edit-group="$emit('editGroup', $event)"
            @delete-group="$emit('deleteGroup', $event)"
            @copy-note="$emit('copy', $event)"
            @edit-note="$emit('editNote', $event)"
            @delete-note="$emit('deleteNote', $event)"
            @reorder-notes="(groupId, event) => $emit('reorderGroupNotes', groupId, event)"
          />
        </div>
      </DragDropProvider>
    </template>
  </div>
</template>

<style scoped>
.list-area {
  flex: 1;
  overflow-y: auto;
  padding: 4px 0;
}

.state-banner {
  margin: 8px 14px 4px;
  padding: 8px 10px;
  border-radius: var(--radius-md);
  background: var(--color-surface);
  color: var(--color-text-secondary);
  font-size: var(--font-size-xs);
}

.section-header {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 14px 4px;
}

.section-header--spaced {
  margin-top: 2px;
  padding-top: 8px;
}

.section-label {
  font-size: var(--font-size-2xs);
  font-weight: 600;
  color: var(--color-text-muted);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.section-badge {
  font-size: 9px;
  font-weight: 500;
  color: var(--color-text-muted);
  background: var(--color-surface);
  padding: 1px 6px;
  border-radius: 8px;
  line-height: 14px;
  font-variant-numeric: tabular-nums;
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  padding: 44px 24px 32px;
}

.empty-icon-wrap {
  width: 48px;
  height: 48px;
  border-radius: 12px;
  background: var(--color-surface);
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--color-text-muted);
  margin-bottom: 4px;
}

.empty-text {
  text-align: center;
  color: var(--color-text-tertiary);
  font-size: var(--font-size-sm);
  font-weight: 500;
}

.empty-hint {
  font-size: var(--font-size-xs);
  color: var(--color-text-muted);
}

.dnd-list {
  display: flex;
  flex-direction: column;
}
</style>
