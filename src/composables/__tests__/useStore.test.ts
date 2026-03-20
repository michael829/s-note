import { beforeEach, describe, expect, it, vi } from "vitest";
import type { Group, Note } from "../useApi";

const groupsFixture: Group[] = [
  { id: 1, name: "Group A", sort_order: 0, created_at: "", updated_at: "" },
];

const notesFixture: Note[] = [
  { id: 1, group_id: null, name: "Note A", content: "A", sort_order: 0, created_at: "", updated_at: "" },
  { id: 2, group_id: 1, name: "Note B", content: "B", sort_order: 1, created_at: "", updated_at: "" },
];

const api = {
  getAllGroups: vi.fn(),
  getAllNotes: vi.fn(),
  createNote: vi.fn(),
  updateNote: vi.fn(),
  deleteNote: vi.fn(),
  updateNotesOrder: vi.fn(),
  updateGroupsOrder: vi.fn(),
  createGroup: vi.fn(),
  updateGroup: vi.fn(),
  deleteGroup: vi.fn(),
  copyToClipboard: vi.fn(),
};

vi.mock("../useApi", () => ({
  useApi: () => api,
}));

describe("useStore", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  async function freshStore() {
    vi.resetModules();
    const module = await import("../useStore");
    return module.useStore();
  }

  it("loads groups and notes", async () => {
    api.getAllGroups.mockResolvedValue(structuredClone(groupsFixture));
    api.getAllNotes.mockResolvedValue(structuredClone(notesFixture));

    const store = await freshStore();
    await store.loadData();

    expect(store.groups.value).toHaveLength(1);
    expect(store.notes.value).toHaveLength(2);
    expect(store.error.value).toBeNull();
  });

  it("adds a note without full reload", async () => {
    const created: Note = { id: 3, group_id: null, name: "Note C", content: "C", sort_order: 2, created_at: "", updated_at: "" };
    api.createNote.mockResolvedValue(created);
    const store = await freshStore();
    store.notes.value = structuredClone(notesFixture);

    await store.addNote("Note C", "C", null);

    expect(store.notes.value[store.notes.value.length - 1]).toEqual(created);
    expect(api.getAllNotes).not.toHaveBeenCalled();
  });

  it("edits a group in place", async () => {
    const updated: Group = { ...groupsFixture[0], name: "Renamed" };
    api.updateGroup.mockResolvedValue(updated);
    const store = await freshStore();
    store.groups.value = structuredClone(groupsFixture);

    await store.editGroup(1, "Renamed");

    expect(store.groups.value[0].name).toBe("Renamed");
  });

  it("removes a group and its notes locally", async () => {
    api.deleteGroup.mockResolvedValue(undefined);
    const store = await freshStore();
    store.groups.value = structuredClone(groupsFixture);
    store.notes.value = structuredClone(notesFixture);

    await store.removeGroup(1);

    expect(store.groups.value).toHaveLength(0);
    expect(store.notes.value).toEqual([notesFixture[0]]);
  });
});
