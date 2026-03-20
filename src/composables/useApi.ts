import { invoke } from "@tauri-apps/api/core";

export interface Group {
  id: number;
  name: string;
  sort_order: number;
  created_at: string;
  updated_at: string;
}

export interface Note {
  id: number;
  group_id: number | null;
  name: string;
  content: string;
  sort_order: number;
  created_at: string;
  updated_at: string;
}

export function useApi() {
  return {
    createGroup: (name: string) => invoke<Group>("create_group", { name }),
    updateGroup: (id: number, name: string) => invoke<Group>("update_group", { id, name }),
    deleteGroup: (id: number) => invoke<void>("delete_group", { id }),
    getGroup: (id: number) => invoke<Group | null>("get_group", { id }),
    getAllGroups: () => invoke<Group[]>("get_all_groups"),

    createNote: (name: string, content: string, groupId: number | null) =>
      invoke<Note>("create_note", { name, content, groupId }),
    updateNote: (id: number, name: string, content: string, groupId: number | null) =>
      invoke<Note>("update_note", { id, name, content, groupId }),
    getNote: (id: number) => invoke<Note | null>("get_note", { id }),
    updateNotesOrder: (noteIds: number[]) =>
      invoke<void>("update_notes_order", { noteIds }),
    updateGroupsOrder: (groupIds: number[]) =>
      invoke<void>("update_groups_order", { groupIds }),
    deleteNote: (id: number) => invoke<void>("delete_note", { id }),
    getAllNotes: () => invoke<Note[]>("get_all_notes"),

    copyToClipboard: (content: string) => invoke<void>("copy_to_clipboard", { content }),

    exportData: () => invoke<string>("export_data"),
    importData: (jsonData: string) => invoke<void>("import_data", { jsonData }),
    saveToFile: (path: string, content: string) => invoke<void>("save_to_file", { path, content }),
    readFile: (path: string) => invoke<string>("read_file", { path }),
    getAppVersion: () => invoke<string>("get_app_version"),
    setIgnoreBlur: (ignore: boolean) => invoke<void>("set_ignore_blur", { ignore }),
    quitApp: () => invoke<void>("quit_app"),
  };
}
