<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useApi } from "./composables/useApi";
import { useStore } from "./composables/useStore";
import { useAutoStart } from "./composables/useAutoStart";
import { useDataTransfer } from "./composables/useDataTransfer";
import { useErrorState } from "./composables/useErrorState";
import ErrorBanner from "./components/ErrorBanner.vue";

const api = useApi();
const store = useStore();
const autoStart = useAutoStart();
const dataTransfer = useDataTransfer();
const errors = useErrorState();

const version = ref("");
const exportSuccess = ref(false);
const importSuccess = ref(false);

onMounted(async () => {
  version.value = await api.getAppVersion();
  await autoStart.load();
});

async function toggleAutoStart() {
  try {
    errors.clearError();
    await autoStart.toggle();
  } catch (e) {
    console.error("Failed to toggle autostart:", e);
    errors.setError(e, "切换开机启动失败");
  }
}

async function handleExport() {
  try {
    errors.clearError();
    const done = await dataTransfer.exportData();
    if (done) {
      exportSuccess.value = true;
      setTimeout(() => { exportSuccess.value = false; }, 2000);
    }
  } catch (e) {
    console.error("Export failed:", e);
    errors.setError(e, "导出失败");
  }
}

async function handleImport() {
  try {
    errors.clearError();
    const done = await dataTransfer.importData();
    if (done) {
      await store.loadData();
      importSuccess.value = true;
      setTimeout(() => { importSuccess.value = false; }, 2000);
    }
  } catch (e) {
    console.error("Import failed:", e);
    errors.setError(e, "导入失败，请确保文件格式正确");
  }
}
</script>

<template>
  <div class="settings">
    <div class="settings-header">
      <h2 class="settings-title">设置</h2>
    </div>

    <div class="settings-body">
      <ErrorBanner :message="errors.errorMessage.value" />
      <!-- App info -->
      <div class="section">
        <div class="section-title">关于</div>
        <div class="setting-row">
          <div class="setting-info">
            <span class="setting-label">S-Note</span>
            <span class="setting-desc">v{{ version }}</span>
          </div>
        </div>
      </div>

      <!-- General settings -->
      <div class="section">
        <div class="section-title">通用</div>
        <div class="setting-row clickable" @click="toggleAutoStart">
          <div class="setting-info">
            <span class="setting-label">开机启动</span>
            <span class="setting-desc">登录时自动启动 S-Note</span>
          </div>
          <div class="toggle" :class="{ active: autoStart.enabled.value }">
            <div class="toggle-knob" />
          </div>
        </div>
      </div>

      <!-- Data -->
      <div class="section">
        <div class="section-title">数据</div>
        <div class="setting-row">
          <div class="setting-info">
            <span class="setting-label">导出数据</span>
            <span class="setting-desc">将所有笔记和分组导出为 JSON 文件</span>
          </div>
          <button class="btn-action" @click="handleExport" :disabled="dataTransfer.exporting.value">
            <template v-if="exportSuccess">
              <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                <polyline points="20 6 9 17 4 12" />
              </svg>
              已导出
            </template>
            <template v-else-if="dataTransfer.exporting.value">导出中...</template>
            <template v-else>
              <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
                <polyline points="7 10 12 15 17 10" />
                <line x1="12" y1="15" x2="12" y2="3" />
              </svg>
              导出
            </template>
          </button>
        </div>

        <div class="setting-row">
          <div class="setting-info">
            <span class="setting-label">导入数据</span>
            <span class="setting-desc">从 JSON 文件恢复笔记和分组</span>
          </div>
          <button class="btn-action btn-secondary" @click="handleImport" :disabled="dataTransfer.importing.value">
            <template v-if="importSuccess">
              <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                <polyline points="20 6 9 17 4 12" />
              </svg>
              已导入
            </template>
            <template v-else-if="dataTransfer.importing.value">导入中...</template>
            <template v-else>
              <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
                <polyline points="17 14 12 9 7 14" />
                <line x1="12" y1="9" x2="12" y2="21" />
              </svg>
              导入
            </template>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<style>
:root {
  --color-bg: #ffffff;
  --color-surface: rgba(0, 0, 0, 0.03);
  --color-surface-hover: rgba(0, 0, 0, 0.05);
  --color-border: rgba(0, 0, 0, 0.08);
  --color-text-primary: #1d1d1f;
  --color-text-secondary: #6e6e73;
  --color-text-tertiary: #aeaeb2;
  --color-accent: #007aff;
  --color-accent-hover: #0062d1;

  --radius-sm: 6px;
  --radius-md: 8px;

  --font-sans: -apple-system, BlinkMacSystemFont, "SF Pro Text", "Helvetica Neue", sans-serif;
  --font-size-xs: 11px;
  --font-size-sm: 12px;
  --font-size-base: 13px;

  --transition-fast: 120ms ease;

  font-family: var(--font-sans);
  font-size: var(--font-size-base);
  color: var(--color-text-primary);
  -webkit-font-smoothing: antialiased;
}

* { margin: 0; padding: 0; box-sizing: border-box; }
body { margin: 0; }
</style>

<style scoped>
.settings {
  height: 100vh;
  display: flex;
  flex-direction: column;
  background: var(--color-bg);
}

.settings-header {
  padding: 16px 20px 0;
}

.settings-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--color-text-primary);
}

.settings-body {
  flex: 1;
  overflow-y: auto;
  padding: 16px 20px;
}

.section {
  margin-bottom: 20px;
}

.section:last-child {
  margin-bottom: 0;
}

.section-title {
  font-size: var(--font-size-xs);
  font-weight: 600;
  color: var(--color-text-tertiary);
  text-transform: uppercase;
  letter-spacing: 0.3px;
  margin-bottom: 8px;
}

.setting-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px 12px;
  background: var(--color-surface);
  border-radius: var(--radius-md);
  margin-bottom: 4px;
}

.setting-row.clickable {
  cursor: pointer;
  transition: background var(--transition-fast);
}

.setting-row.clickable:hover {
  background: var(--color-surface-hover);
}

.setting-info {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.setting-label {
  font-size: var(--font-size-base);
  font-weight: 500;
  color: var(--color-text-primary);
}

.setting-desc {
  font-size: var(--font-size-xs);
  color: var(--color-text-tertiary);
}

.toggle {
  width: 36px;
  height: 20px;
  background: rgba(0, 0, 0, 0.12);
  border-radius: 10px;
  position: relative;
  transition: background 200ms ease;
  flex-shrink: 0;
}

.toggle.active {
  background: var(--color-accent);
}

.toggle-knob {
  width: 16px;
  height: 16px;
  background: white;
  border-radius: 50%;
  position: absolute;
  top: 2px;
  left: 2px;
  transition: transform 200ms ease;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.15);
}

.toggle.active .toggle-knob {
  transform: translateX(16px);
}

.btn-action {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 5px 12px;
  border: none;
  border-radius: var(--radius-sm);
  font-size: var(--font-size-sm);
  font-family: var(--font-sans);
  font-weight: 500;
  cursor: pointer;
  background: var(--color-accent);
  color: #fff;
  transition: background var(--transition-fast), opacity var(--transition-fast);
  flex-shrink: 0;
}

.btn-action:hover:not(:disabled) {
  background: var(--color-accent-hover);
}

.btn-action:disabled {
  opacity: 0.6;
  cursor: default;
}
</style>
