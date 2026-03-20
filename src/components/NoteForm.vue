<script setup lang="ts">
import { ref, onMounted, nextTick } from "vue";
import type { Group } from "../composables/useApi";

defineProps<{
  groups: Group[];
  disabled?: boolean;
}>();

const emit = defineEmits<{
  save: [name: string, content: string, groupId: number | null];
  createGroup: [name: string];
}>();

const name = ref("");
const content = ref("");
const selectedGroupId = ref<number | null>(null);
const showNewGroupInput = ref(false);
const newGroupName = ref("");
const contentRef = ref<HTMLTextAreaElement>();

onMounted(() => {
  contentRef.value?.focus();
});

function handleKeydown(e: KeyboardEvent) {
  if ((e.metaKey || e.ctrlKey) && e.key === "Enter") {
    e.preventDefault();
    save();
  }
}

function save() {
  const trimmedName = name.value.trim();
  const trimmedContent = content.value.trim();
  if (!trimmedName || !trimmedContent) return;
  emit("save", trimmedName, trimmedContent, selectedGroupId.value);
  name.value = "";
  content.value = "";
  nextTick(() => contentRef.value?.focus());
}

function onGroupChange(e: Event) {
  const val = (e.target as HTMLSelectElement).value;
  if (val === "__new__") {
    showNewGroupInput.value = true;
    selectedGroupId.value = null;
    (e.target as HTMLSelectElement).value = "";
  } else {
    selectedGroupId.value = val ? Number(val) : null;
  }
}

function confirmNewGroup() {
  const trimmed = newGroupName.value.trim();
  if (trimmed) {
    emit("createGroup", trimmed);
  }
  showNewGroupInput.value = false;
  newGroupName.value = "";
}

function cancelNewGroup() {
  showNewGroupInput.value = false;
  newGroupName.value = "";
}

</script>

<template>
  <div class="form-area" @keydown="handleKeydown">
    <div class="form-card">
      <input
        v-model="name"
        type="text"
        placeholder="标题"
        class="form-title"
        :disabled="disabled"
      />
      <div class="form-divider" />
      <textarea
        ref="contentRef"
        v-model="content"
        placeholder="写点什么..."
        class="form-content"
        :disabled="disabled"
      />
    </div>
    <div class="form-footer">
      <div class="group-select-wrap">
        <span class="group-dot" />
        <select class="group-native-select" @change="onGroupChange" :disabled="disabled">
          <option value="">默认分组</option>
          <option v-for="group in groups" :key="group.id" :value="group.id">
            {{ group.name }}
          </option>
          <option value="__new__">+ 新建分组</option>
        </select>
      </div>
      <div class="form-actions">
        <span class="kbd">⌘↵</span>
        <button class="btn-save" @click="save" :disabled="disabled || !name.trim() || !content.trim()">保存</button>
      </div>
    </div>

    <!-- New group dialog -->
    <Transition name="fade">
      <div v-if="showNewGroupInput" class="overlay" @click.self="cancelNewGroup">
        <div class="dialog">
          <p class="dialog-title">新建分组</p>
          <input
            v-model="newGroupName"
            type="text"
            placeholder="输入分组名称"
            class="dialog-input"
            autofocus
            @keydown.enter="confirmNewGroup"
            @keydown.escape="cancelNewGroup"
          />
          <div class="dialog-buttons">
            <button class="btn-ghost" @click="cancelNewGroup">取消</button>
            <button class="btn-primary-sm" @click="confirmNewGroup">确定</button>
          </div>
        </div>
      </div>
    </Transition>
  </div>
</template>

<style scoped>
.form-area {
  padding: 14px 14px 10px;
  border-bottom: 0.5px solid var(--color-border);
  position: relative;
}

.form-card {
  background: var(--color-card);
  border-radius: var(--radius-lg);
  border: 0.5px solid var(--color-card-border);
  overflow: hidden;
  box-shadow: var(--color-card-shadow);
}

.form-title {
  width: 100%;
  border: none;
  background: transparent;
  font-size: var(--font-size-md);
  font-weight: 700;
  color: var(--color-text-primary);
  outline: none;
  font-family: var(--font-sans);
  padding: 10px 12px 0;
  letter-spacing: -0.1px;
}

.form-title::placeholder {
  color: var(--color-text-muted);
  font-weight: 600;
}

.form-divider {
  height: 0.5px;
  background: rgba(0, 0, 0, 0.05);
  margin: 6px 12px;
}

.form-content {
  width: 100%;
  border: none;
  background: transparent;
  font-size: var(--font-size-sm);
  color: var(--color-text-secondary);
  outline: none;
  font-family: var(--font-sans);
  padding: 0 12px 10px;
  resize: none;
  min-height: 100px;
  line-height: 1.7;
}

.form-content::placeholder {
  color: var(--color-text-muted);
}

.form-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: 8px;
}

.group-select-wrap {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 3px 8px;
  background: white;
  border: 0.5px solid rgba(0, 0, 0, 0.08);
  border-radius: var(--radius-sm);
  position: relative;
  cursor: pointer;
}

.group-dot {
  width: 5px;
  height: 5px;
  border-radius: 50%;
  flex-shrink: 0;
}

.group-native-select {
  border: none;
  background: transparent;
  font-size: var(--font-size-xs);
  font-family: var(--font-sans);
  color: var(--color-text-tertiary);
  outline: none;
  cursor: pointer;
  padding-right: 2px;
  -webkit-appearance: none;
  appearance: none;
}

.form-actions {
  display: flex;
  align-items: center;
  gap: 8px;
}

.kbd {
  font-size: var(--font-size-2xs);
  color: var(--color-text-muted);
}

.btn-save {
  background: var(--color-accent);
  color: var(--color-bg);
  border: none;
  border-radius: var(--radius-sm);
  padding: 4px 14px;
  font-size: 11px;
  font-weight: 500;
  font-family: var(--font-sans);
  cursor: pointer;
  transition: background var(--transition-fast), opacity var(--transition-fast);
}

.btn-save:hover:not(:disabled) {
  background: var(--color-accent-hover);
}

.btn-save:disabled {
  opacity: 0.3;
  cursor: default;
}

/* Dialog */
.overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.15);
  backdrop-filter: blur(6px);
  -webkit-backdrop-filter: blur(6px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 10;
}

.dialog {
  background: var(--color-bg);
  padding: 16px 18px;
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-lg);
  width: 240px;
  border: 0.5px solid var(--color-border);
}

.dialog-title {
  font-size: var(--font-size-base);
  font-weight: 600;
  color: var(--color-text-primary);
  margin-bottom: 10px;
}

.dialog-input {
  width: 100%;
  padding: 6px 10px;
  border: 0.5px solid var(--color-border-strong);
  border-radius: var(--radius-sm);
  font-size: var(--font-size-sm);
  font-family: var(--font-sans);
  outline: none;
  background: white;
  color: var(--color-text-primary);
  transition: border-color var(--transition-fast), box-shadow var(--transition-fast);
}

.dialog-input::placeholder {
  color: var(--color-text-muted);
}

.dialog-input:focus {
  border-color: var(--color-accent);
  box-shadow: 0 0 0 3px var(--color-accent-subtle);
}

.dialog-buttons {
  display: flex;
  gap: 6px;
  justify-content: flex-end;
  margin-top: 10px;
}

.btn-ghost {
  padding: 4px 10px;
  border: none;
  border-radius: var(--radius-xs);
  font-size: 11px;
  font-family: var(--font-sans);
  font-weight: 500;
  cursor: pointer;
  background: none;
  color: var(--color-text-tertiary);
  transition: background var(--transition-fast), color var(--transition-fast);
}

.btn-ghost:hover {
  background: var(--color-surface-hover);
  color: var(--color-text-primary);
}

.btn-primary-sm {
  padding: 4px 14px;
  border: none;
  border-radius: var(--radius-xs);
  font-size: 11px;
  font-family: var(--font-sans);
  font-weight: 500;
  cursor: pointer;
  background: var(--color-accent);
  color: var(--color-bg);
  transition: background var(--transition-fast);
}

.btn-primary-sm:hover {
  background: var(--color-accent-hover);
}

.fade-enter-active, .fade-leave-active { transition: opacity var(--transition-normal); }
.fade-enter-from, .fade-leave-to { opacity: 0; }
</style>
