<script setup lang="ts">
defineProps<{
  totalCount: number;
  groupCount: number;
  autoStartEnabled: boolean;
  importing: boolean;
  exporting: boolean;
  busy: boolean;
}>();

defineEmits<{
  toggleAutoStart: [];
  importData: [];
  exportData: [];
  quit: [];
}>();
</script>

<template>
  <div class="status-bar">
    <div class="status-info">
      <template v-if="totalCount > 0">
        <span>{{ totalCount }} 条笔记</span>
        <span v-if="groupCount > 0" class="status-sep">&middot;</span>
        <span v-if="groupCount > 0">{{ groupCount }} 个分组</span>
      </template>
      <span v-else-if="busy">同步中...</span>
    </div>
    <div class="status-actions">
      <button
        class="action-btn"
        :class="{ 'action-btn--active': autoStartEnabled }"
        @click="$emit('toggleAutoStart')"
        title="开机启动"
        aria-label="开机启动"
      >
        <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="12" cy="12" r="10" fill="none" />
          <circle v-if="autoStartEnabled" cx="12" cy="12" r="6" fill="currentColor" stroke="none" />
        </svg>
        开机启动
      </button>
      <button class="action-btn" @click="$emit('importData')" :disabled="importing || busy" title="导入数据" aria-label="导入">
        <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
          <polyline points="14 2 14 8 20 8" />
          <path d="M12 11v7" />
          <polyline points="9 15 12 18 15 15" />
        </svg>
        导入
      </button>
      <button class="action-btn" @click="$emit('exportData')" :disabled="exporting || busy" title="导出数据" aria-label="导出">
        <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
          <polyline points="14 2 14 8 20 8" />
          <path d="M12 18v-7" />
          <polyline points="9 14 12 11 15 14" />
        </svg>
        导出
      </button>
      <button class="action-btn" @click="$emit('quit')" title="退出 S-Note" aria-label="退出">
        <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
          <path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4" />
          <polyline points="16 17 21 12 16 7" />
          <line x1="21" y1="12" x2="9" y2="12" />
        </svg>
        退出
      </button>
    </div>
  </div>
</template>

<style scoped>
.status-bar {
  padding: 6px 14px;
  border-top: 0.5px solid rgba(0, 0, 0, 0.05);
  font-size: var(--font-size-2xs);
  color: var(--color-text-muted);
  display: flex;
  align-items: center;
  justify-content: space-between;
  flex-shrink: 0;
  font-variant-numeric: tabular-nums;
  background: linear-gradient(180deg, #f5f3f0 0%, #f0ede9 100%);
  border-radius: 0 0 var(--radius-xl) var(--radius-xl);
}

.status-info {
  display: flex;
  gap: 4px;
}

.status-sep {
  opacity: 0.5;
}

.status-actions {
  display: flex;
  align-items: center;
  gap: 4px;
}

.action-btn {
  display: flex;
  align-items: center;
  gap: 3px;
  border: none;
  border-radius: var(--radius-xs);
  background: none;
  color: var(--color-text-primary);
  cursor: pointer;
  font-family: var(--font-sans);
  font-size: var(--font-size-2xs);
  padding: 2px 4px;
  transition: all var(--transition-fast);
}

.action-btn:hover:not(:disabled) {
  background: var(--color-surface-hover);
  color: var(--color-text-secondary);
}

.action-btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

.action-btn--active {
  color: var(--color-success);
  background: rgba(34, 197, 94, 0.08);
}

.action-btn--active:hover {
  background: rgba(34, 197, 94, 0.12) !important;
  color: var(--color-success) !important;
}
</style>
