<script setup lang="ts">
import { onMounted, ref, computed } from "vue";
import { useStore } from "./composables/useStore";
import type { Note, Group } from "./composables/useApi";
import NoteForm from "./components/NoteForm.vue";
import NoteItem from "./components/NoteItem.vue";
import GroupItem from "./components/GroupItem.vue";
import ConfirmDialog from "./components/ConfirmDialog.vue";
import Toast from "./components/Toast.vue";
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { save, open } from "@tauri-apps/plugin-dialog";
import { enable, disable, isEnabled } from "@tauri-apps/plugin-autostart";

const store = useStore();

const confirmVisible = ref(false);
const confirmMessage = ref("");
let confirmCallback: (() => void) | null = null;

const editorOpen = ref(false);

const toastVisible = ref(false);
const toastMessage = ref("");
let toastTimer: ReturnType<typeof setTimeout> | null = null;

const autoStart = ref(false);
const exporting = ref(false);
const importing = ref(false);

function showToast(msg: string) {
  toastMessage.value = msg;
  toastVisible.value = true;
  if (toastTimer) clearTimeout(toastTimer);
  toastTimer = setTimeout(() => {
    toastVisible.value = false;
  }, 1500);
}

const totalCount = computed(() => store.notes.value.length);

onMounted(async () => {
  await store.loadData();

  await getCurrentWindow().setFocus();
  // 尝试让窗口获取焦点. 否则打开窗口可能需要点第二次才会有效(当然这个代码好像不是每次都有效, 不会tauri暂时就先这样了)
  
  try {
    autoStart.value = await isEnabled();
  } catch (_) {
    autoStart.value = false;
  }

  document.addEventListener("keydown", (e) => {
    if (e.key === "Escape") {
      getCurrentWindow().hide();
    }
  });

  window.addEventListener("note-data-changed", () => {
    store.loadData();
  });

  // Listen for tray export event
  listen("tray-export-data", async () => {
    await handleExportData();
  });
});

async function toggleAutoStart() {
  try {
    if (autoStart.value) {
      await disable();
      autoStart.value = false;
      showToast("已关闭开机自启");
    } else {
      await enable();
      autoStart.value = true;
      showToast("已开启开机自启");
    }
  } catch (e) {
    console.error("Failed to toggle autostart:", e);
  }
}

async function handleExport() {
  try {
    exporting.value = true;
    const data = await invoke<string>("export_data");
    const filePath = await save({
      defaultPath: `s-note-export.json`,
      filters: [{ name: "JSON", extensions: ["json"] }],
    });
    if (filePath) {
      await invoke("save_to_file", { path: filePath, content: data });
      showToast("导出成功");
    }
  } catch (e) {
    console.error("Export failed:", e);
  } finally {
    exporting.value = false;
  }
}

async function handleImport() {
  try {
    const filePath = await open({
      multiple: false,
      filters: [{ name: "JSON", extensions: ["json"] }],
    });
    
    if (filePath) {
      importing.value = true;
      const content = await invoke<string>("read_file", { path: filePath });
      await invoke("import_data", { jsonData: content });
      await store.loadData();
      showToast("导入成功");
    }
  } catch (e) {
    console.error("Import failed:", e);
    showToast("导入失败");
  } finally {
    importing.value = false;
  }
}

async function handleSave(name: string, content: string, groupId: number | null) {
  await store.addNote(name, content, groupId);
  showToast("已保存");
}

async function handleCreateGroup(name: string) {
  await store.addGroup(name);
}

function handleCopy(note: Note) {
  store.copyContent(note.content);
  showToast("已复制到剪贴板");
  // Hide window after copy
  getCurrentWindow().hide();
}

function handleDeleteNote(note: Note) {
  confirmMessage.value = `确认删除「${note.name}」？`;
  confirmCallback = async () => {
    await store.removeNote(note.id);
    confirmVisible.value = false;
    showToast("已删除");
  };
  confirmVisible.value = true;
}

function handleDeleteGroup(group: Group) {
  confirmMessage.value = `确认删除分组「${group.name}」及其所有笔记？`;
  confirmCallback = async () => {
    await store.removeGroup(group.id);
    confirmVisible.value = false;
    showToast("已删除");
  };
  confirmVisible.value = true;
}

function handleEditNote(note: Note) {
  openEditor("note", note);
}

function handleEditGroup(group: Group) {
  openEditor("group", group);
}

async function openEditor(type: string, data: Note | Group) {
  editorOpen.value = true;
  const editor = new WebviewWindow("editor", {
    url: `editor.html?type=${type}&id=${data.id}`,
    title: type === "note" ? "编辑笔记" : "编辑分组",
    width: 420,
    height: type === "note" ? 380 : 220,
    resizable: false,
    center: true,
    alwaysOnTop: true,
  });

  editor.once("tauri://destroyed", async () => {
    editorOpen.value = false;
    await store.loadData();
  });
}

function cancelConfirm() {
  confirmVisible.value = false;
  confirmCallback = null;
}

function doConfirm() {
  if (confirmCallback) confirmCallback();
}

async function handleExportData() {
  try {
    const data = await invoke<string>("export_data");
    const filePath = await save({
      defaultPath: "s-note-export.json",
      filters: [{ name: "JSON", extensions: ["json"] }],
    });
    if (filePath) {
      await invoke("save_to_file", { path: filePath, content: data });
      showToast("导出成功");
    }
  } catch (e) {
    console.error("Export failed:", e);
  }
}

function handleQuit() {
  invoke("quit_app");
}
</script>

<template>
  <div class="panel">
      <NoteForm
        :groups="store.groups.value"
        @save="handleSave"
        @create-group="handleCreateGroup"
      />

      <div class="list-area">
        <template v-if="store.ungroupedNotes.value.length === 0 && store.groups.value.length === 0">
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

        <!-- Ungrouped notes -->
        <template v-if="store.ungroupedNotes.value.length > 0">
          <div class="section-header">
            <span class="section-label">笔记</span>
            <span class="section-badge">{{ store.ungroupedNotes.value.length }}</span>
          </div>
          <NoteItem
            v-for="note in store.ungroupedNotes.value"
            :key="note.id"
            :note="note"
            @copy="handleCopy"
            @edit="handleEditNote"
            @delete="handleDeleteNote"
          />
        </template>

        <!-- Groups -->
        <template v-if="store.groups.value.length > 0">
          <div class="section-header" :class="{ 'section-header--spaced': store.ungroupedNotes.value.length > 0 }">
            <span class="section-label">分组</span>
            <span class="section-badge">{{ store.groups.value.length }}</span>
          </div>
          <GroupItem
            v-for="group in store.groups.value"
            :key="group.id"
            :group="group"
            :notes="store.getNotesForGroup(group.id)"
            @edit-group="handleEditGroup"
            @delete-group="handleDeleteGroup"
            @copy-note="handleCopy"
            @edit-note="handleEditNote"
            @delete-note="handleDeleteNote"
          />
        </template>
      </div>

      <!-- Status bar -->
      <div class="status-bar">
        <div class="status-info">
          <template v-if="totalCount > 0">
            <span>{{ totalCount }} 条笔记</span>
            <span v-if="store.groups.value.length > 0" class="status-sep">&middot;</span>
            <span v-if="store.groups.value.length > 0">{{ store.groups.value.length }} 个分组</span>
          </template>
        </div>
        <div class="status-actions">
          <button class="action-btn" :class="{ 'action-btn--active': autoStart }" @click="toggleAutoStart" title="开机启动" aria-label="开机启动">
            <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <circle cx="12" cy="12" r="10" fill="none" />
              <circle cx="12" cy="12" r="6" fill="currentColor" stroke="none" v-if="autoStart" />
            </svg>
            开机启动
          </button>
          <button class="action-btn" @click="handleImport" :disabled="importing" title="导入数据" aria-label="导入">
            <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
              <polyline points="17 14 12 9 7 14" />
              <line x1="12" y1="9" x2="12" y2="21" />
            </svg>
            导入
          </button>
          <button class="action-btn" @click="handleExport" :disabled="exporting" title="导出数据" aria-label="导出">
            <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
              <polyline points="7 10 12 15 17 10" />
              <line x1="12" y1="15" x2="12" y2="3" />
            </svg>
            导出
          </button>
          <button class="action-btn" @click="handleQuit" title="退出 S-Note" aria-label="退出">
            <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
              <path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4" />
              <polyline points="16 17 21 12 16 7" />
              <line x1="21" y1="12" x2="9" y2="12" />
            </svg>
            退出
          </button>
        </div>
      </div>

    <!-- Block interaction when editor is open -->
    <div v-if="editorOpen" class="editor-overlay" />

    <ConfirmDialog
      :visible="confirmVisible"
      :message="confirmMessage"
      @confirm="doConfirm"
      @cancel="cancelConfirm"
    />

    <Toast
      :visible="toastVisible"
      :message="toastMessage"
      @hide="toastVisible = false"
    />
  </div>
</template>

<style>
:root {
  --color-bg: #faf9f7;
  --color-bg-solid: #ffffff;
  --color-surface: rgba(0, 0, 0, 0.03);
  --color-surface-hover: rgba(0, 0, 0, 0.04);
  --color-surface-active: rgba(0, 0, 0, 0.07);
  --color-border: rgba(0, 0, 0, 0.06);
  --color-border-strong: rgba(0, 0, 0, 0.1);
  --color-border-focus: #2c2825;
  --color-text-primary: #2c2825;
  --color-text-secondary: #5a5550;
  --color-text-tertiary: #8a8680;
  --color-text-muted: #c5c0b8;
  --color-accent: #2c2825;
  --color-accent-hover: #3d3830;
  --color-accent-subtle: rgba(44, 40, 37, 0.06);
  --color-danger: #e74c3c;
  --color-danger-hover: #d63028;
  --color-danger-subtle: rgba(231, 76, 60, 0.06);
  --color-success: #22c55e;

  --color-card: #ffffff;
  --color-card-border: rgba(0, 0, 0, 0.06);
  --color-card-shadow: 0 1px 3px rgba(0, 0, 0, 0.03);

  --shadow-panel: 0 0 0 0.5px rgba(0, 0, 0, 0.06),
                  0 20px 60px rgba(0, 0, 0, 0.15),
                  0 4px 16px rgba(0, 0, 0, 0.08);
  --shadow-sm: 0 1px 2px rgba(0, 0, 0, 0.03);
  --shadow-md: 0 4px 12px rgba(0, 0, 0, 0.06);
  --shadow-lg: 0 12px 32px rgba(0, 0, 0, 0.1);

  --scrollbar-thumb: rgba(0, 0, 0, 0.08);
  --scrollbar-thumb-hover: rgba(0, 0, 0, 0.15);

  --radius-xs: 4px;
  --radius-sm: 6px;
  --radius-md: 8px;
  --radius-lg: 10px;
  --radius-xl: 14px;

  --font-sans: -apple-system, BlinkMacSystemFont, "SF Pro Text", "Helvetica Neue", sans-serif;
  --font-size-2xs: 9px;
  --font-size-xs: 10px;
  --font-size-sm: 12px;
  --font-size-base: 13px;
  --font-size-md: 14px;

  --transition-fast: 120ms ease;
  --transition-normal: 180ms ease;
  --transition-slow: 280ms cubic-bezier(0.2, 0, 0, 1);

  font-family: var(--font-sans);
  font-size: var(--font-size-base);
  color: var(--color-text-primary);
  background-color: transparent;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

@media (prefers-reduced-motion: reduce) {
  *, *::before, *::after {
    animation-duration: 0.01ms !important;
    transition-duration: 0.01ms !important;
  }
}

* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html, body, #app {
  margin: 0;
  overflow: hidden;
  background: transparent !important;
}

::-webkit-scrollbar {
  width: 5px;
}
::-webkit-scrollbar-track {
  background: transparent;
}
::-webkit-scrollbar-thumb {
  background: var(--scrollbar-thumb);
  border-radius: 3px;
}
::-webkit-scrollbar-thumb:hover {
  background: var(--scrollbar-thumb-hover);
}

::selection {
  background: rgba(44, 40, 37, 0.15);
}
</style>

<style scoped>
.panel {
  width: 100%;
  height: 100vh;
  background: var(--color-bg);
  border-radius: var(--radius-xl);
  box-shadow: var(--shadow-panel);
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.list-area {
  flex: 1;
  overflow-y: auto;
  padding: 4px 0;
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

.action-btn--danger:hover {
  background: var(--color-danger-subtle);
  color: var(--color-danger);
}

.editor-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.08);
  z-index: 9999;
}
</style>
