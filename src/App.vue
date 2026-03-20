<script setup lang="ts">
import { onMounted, onUnmounted, ref, computed } from "vue";
import { useStore } from "./composables/useStore";
import { useApi } from "./composables/useApi";
import type { Note, Group } from "./composables/useApi";
import NoteForm from "./components/NoteForm.vue";
import NoteSections from "./components/NoteSections.vue";
import StatusBar from "./components/StatusBar.vue";
import ConfirmDialog from "./components/ConfirmDialog.vue";
import Toast from "./components/Toast.vue";
import { useToast } from "./composables/useToast";
import { useAutoStart } from "./composables/useAutoStart";
import { useDataTransfer } from "./composables/useDataTransfer";
import { useEditorSync } from "./composables/useEditorSync";
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { listen } from "@tauri-apps/api/event";

const api = useApi();
const store = useStore();
const toast = useToast();
const autoStart = useAutoStart();
const dataTransfer = useDataTransfer();
const editorSync = useEditorSync();

const confirmVisible = ref(false);
const confirmMessage = ref("");
const editorOpen = ref(false);
let confirmCallback: (() => void) | null = null;
let stopSyncListener: (() => void) | null = null;
let stopTrayListener: (() => void) | null = null;

// 持久化笔记排序
async function onNoteDragEnd(event: any) {
  if (!event.operation) return;
  await store.reorderUngroupedNotes(event);
  toast.show("顺序已更新");
}

// 持久化分组排序
async function onGroupDragEnd(event: any) {
  if (!event.operation) return;
  await store.reorderGroups(event);
  toast.show("分组顺序已更新");
}

const totalCount = computed(() => store.notes.value.length);
const ungroupedNotesModel = computed(() => store.ungroupedNotes.value);
const groupsModel = computed(() => store.groups.value);
const notesByGroup = computed(() =>
  Object.fromEntries(groupsModel.value.map((group) => [group.id, store.getNotesForGroup(group.id)]))
);

onMounted(async () => {
  await store.loadData();
  await getCurrentWindow().setFocus();
  await autoStart.load();

  document.addEventListener("keydown", (e) => {
    if (e.key === "Escape") {
      getCurrentWindow().hide();
    }
  });

  stopSyncListener = await editorSync.onDataChanged(async () => {
    await store.loadData();
  });

  stopTrayListener = await listen("tray-export-data", async () => {
    await handleExport();
  });
});

onUnmounted(() => {
  stopSyncListener?.();
  stopTrayListener?.();
});

async function toggleAutoStart() {
  try {
    const enabled = await autoStart.toggle();
    toast.show(enabled ? "已开启开机自启" : "已关闭开机自启");
  } catch (e) {
    console.error("Failed to toggle autostart:", e);
  }
}

async function handleExport() {
  try {
    const done = await dataTransfer.exportData();
    if (done) {
      toast.show("导出成功");
    }
  } catch (e) {
    console.error("Export failed:", e);
  }
}

async function handleImport() {
  try {
    const done = await dataTransfer.importData();
    if (done) {
      await store.loadData();
      toast.show("导入成功");
    }
  } catch (e) {
    console.error("Import failed:", e);
    toast.show("导入失败");
  }
}

async function handleSave(name: string, content: string, groupId: number | null) {
  await store.addNote(name, content, groupId);
  toast.show("已保存");
}

async function handleCreateGroup(name: string) {
  await store.addGroup(name);
}

async function handleCopy(note: Note) {
  await store.copyContent(note.content);
  toast.show("已复制到剪贴板");
  await getCurrentWindow().hide();
}

function handleDeleteNote(note: Note) {
  confirmMessage.value = `确认删除「${note.name}」？`;
  confirmCallback = async () => {
    await store.removeNote(note.id);
    confirmVisible.value = false;
    toast.show("已删除");
  };
  confirmVisible.value = true;
}

function handleDeleteGroup(group: Group) {
  confirmMessage.value = `确认删除分组「${group.name}」及其所有笔记？`;
  confirmCallback = async () => {
    await store.removeGroup(group.id);
    confirmVisible.value = false;
    toast.show("已删除");
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

async function handleGroupNoteReorder(groupId: number, event: any) {
  if (!event.operation) return;
  await store.reorderGroupNotes(groupId, event);
  toast.show("顺序已更新");
}

function cancelConfirm() {
  confirmVisible.value = false;
  confirmCallback = null;
}

function doConfirm() {
  if (confirmCallback) confirmCallback();
}

function handleQuit() {
  void api.quitApp();
}
</script>

<template>
  <div class="panel">
    <NoteForm
      :groups="store.groups.value"
      :disabled="store.busy.value"
      @save="handleSave"
      @create-group="handleCreateGroup"
    />

    <NoteSections
      :groups="groupsModel"
      :ungrouped-notes="ungroupedNotesModel"
      :notes-by-group="notesByGroup"
      :loading="store.isLoading.value"
      :error="store.error.value"
      @copy="handleCopy"
      @edit-note="handleEditNote"
      @delete-note="handleDeleteNote"
      @edit-group="handleEditGroup"
      @delete-group="handleDeleteGroup"
      @reorder-ungrouped="onNoteDragEnd"
      @reorder-groups="onGroupDragEnd"
      @reorder-group-notes="handleGroupNoteReorder"
    />

    <StatusBar
      :total-count="totalCount"
      :group-count="store.groups.value.length"
      :auto-start-enabled="autoStart.enabled.value"
      :importing="dataTransfer.importing.value"
      :exporting="dataTransfer.exporting.value"
      :busy="store.busy.value"
      @toggle-auto-start="toggleAutoStart"
      @import-data="handleImport"
      @export-data="handleExport"
      @quit="handleQuit"
    />

    <div v-if="editorOpen" class="editor-overlay" />

    <ConfirmDialog
      :visible="confirmVisible"
      :message="confirmMessage"
      @confirm="doConfirm"
      @cancel="cancelConfirm"
    />

    <Toast
      :visible="toast.visible.value"
      :message="toast.message.value"
      @hide="toast.hide"
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

.editor-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.08);
  z-index: 9999;
}
</style>
