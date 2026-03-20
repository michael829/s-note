import { beforeEach, describe, expect, it, vi } from "vitest";
import { mount } from "@vue/test-utils";
import { nextTick, ref, computed } from "vue";
import type { Group, Note } from "../composables/useApi";

class ResizeObserverMock {
  observe() {}
  unobserve() {}
  disconnect() {}
}

vi.stubGlobal("ResizeObserver", ResizeObserverMock);

const groups = ref<Group[]>([
  { id: 1, name: "Work", sort_order: 0, created_at: "", updated_at: "" },
]);
const notes = ref<Note[]>([
  { id: 1, group_id: null, name: "Alpha", content: "A", sort_order: 0, created_at: "", updated_at: "" },
  { id: 2, group_id: 1, name: "Beta", content: "B", sort_order: 1, created_at: "", updated_at: "" },
]);
const store = {
  groups,
  notes,
  isLoading: ref(false),
  isMutating: ref(false),
  busy: computed(() => false),
  error: ref<string | null>(null),
  ungroupedNotes: computed(() => notes.value.filter((note) => note.group_id === null)),
  getNotesForGroup: (groupId: number) => notes.value.filter((note) => note.group_id === groupId),
  loadData: vi.fn(),
  addNote: vi.fn(async (name: string, content: string, groupId: number | null) => {
    notes.value = [
      ...notes.value,
      {
        id: notes.value.length + 10,
        group_id: groupId,
        name,
        content,
        sort_order: notes.value.length,
        created_at: "",
        updated_at: "",
      },
    ];
  }),
  editNote: vi.fn(),
  removeNote: vi.fn(async (id: number) => {
    notes.value = notes.value.filter((note) => note.id !== id);
  }),
  updateNotesOrder: vi.fn(),
  addGroup: vi.fn(async (name: string) => {
    groups.value = [
      ...groups.value,
      { id: groups.value.length + 10, name, sort_order: groups.value.length, created_at: "", updated_at: "" },
    ];
  }),
  editGroup: vi.fn(),
  removeGroup: vi.fn(async (id: number) => {
    groups.value = groups.value.filter((group) => group.id !== id);
    notes.value = notes.value.filter((note) => note.group_id !== id);
  }),
  updateGroupsOrder: vi.fn(),
  copyContent: vi.fn(),
  reorderUngroupedNotes: vi.fn(),
  reorderGroupNotes: vi.fn(),
  reorderGroups: vi.fn(),
};

const toast = {
  visible: ref(false),
  message: ref(""),
  show: vi.fn((message: string) => {
    toast.message.value = message;
    toast.visible.value = true;
  }),
  hide: vi.fn(),
};

const autoStart = {
  enabled: ref(false),
  load: vi.fn(),
  toggle: vi.fn(async () => {
    autoStart.enabled.value = !autoStart.enabled.value;
    return autoStart.enabled.value;
  }),
};

const dataTransfer = {
  importing: ref(false),
  exporting: ref(false),
  importData: vi.fn(async () => true),
  exportData: vi.fn(async () => true),
};

const editorSync = {
  onDataChanged: vi.fn(async () => () => {}),
};

const getCurrentWindow = vi.fn(() => ({
  setFocus: vi.fn(),
  hide: vi.fn(),
}));

vi.mock("../composables/useStore", () => ({
  useStore: () => store,
}));

vi.mock("../composables/useToast", () => ({
  useToast: () => toast,
}));

vi.mock("../composables/useAutoStart", () => ({
  useAutoStart: () => autoStart,
}));

vi.mock("../composables/useDataTransfer", () => ({
  useDataTransfer: () => dataTransfer,
}));

vi.mock("../composables/useEditorSync", () => ({
  useEditorSync: () => editorSync,
}));

vi.mock("../composables/useApi", async () => {
  const actual = await vi.importActual("../composables/useApi");
  return {
    ...actual,
    useApi: () => ({
      quitApp: vi.fn(),
    }),
  };
});

vi.mock("@tauri-apps/api/window", () => ({
  getCurrentWindow,
}));

vi.mock("@tauri-apps/api/event", () => ({
  listen: vi.fn(async () => () => {}),
}));

vi.mock("@tauri-apps/api/webviewWindow", () => ({
  WebviewWindow: vi.fn(() => ({
    once: vi.fn(),
  })),
}));

describe("App flow", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    groups.value = [{ id: 1, name: "Work", sort_order: 0, created_at: "", updated_at: "" }];
    notes.value = [
      { id: 1, group_id: null, name: "Alpha", content: "A", sort_order: 0, created_at: "", updated_at: "" },
      { id: 2, group_id: 1, name: "Beta", content: "B", sort_order: 1, created_at: "", updated_at: "" },
    ];
  });

  it("creates a note from the main form", async () => {
    const { default: App } = await import("../App.vue");
    const wrapper = mount(App);

    await wrapper.find('input[placeholder="标题"]').setValue("Gamma");
    await wrapper.find("textarea").setValue("Body");
    await wrapper.find("button.btn-save").trigger("click");
    await nextTick();

    expect(store.addNote).toHaveBeenCalledWith("Gamma", "Body", null);
    expect(notes.value.some((note) => note.name === "Gamma")).toBe(true);
  });

  it("toggles auto start from the status bar", async () => {
    const { default: App } = await import("../App.vue");
    const wrapper = mount(App);

    await wrapper.get('button[title="开机启动"]').trigger("click");

    expect(autoStart.toggle).toHaveBeenCalled();
  });

  it("imports and exports data from the status bar", async () => {
    const { default: App } = await import("../App.vue");
    const wrapper = mount(App);

    await wrapper.get('button[title="导入数据"]').trigger("click");
    await wrapper.get('button[title="导出数据"]').trigger("click");

    expect(dataTransfer.importData).toHaveBeenCalled();
    expect(dataTransfer.exportData).toHaveBeenCalled();
  });
});
