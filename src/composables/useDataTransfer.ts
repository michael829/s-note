import { ref } from "vue";
import { open, save } from "@tauri-apps/plugin-dialog";
import { useApi } from "./useApi";
import { useEditorSync } from "./useEditorSync";

const api = useApi();
const { notifyDataChanged } = useEditorSync();

const exporting = ref(false);
const importing = ref(false);

export function useDataTransfer() {
  async function exportData() {
    if (exporting.value || importing.value) {
      return false;
    }

    try {
      exporting.value = true;
      await api.setIgnoreBlur(true);
      const data = await api.exportData();
      const filePath = await save({
        defaultPath: "s-note-export.json",
        filters: [{ name: "JSON", extensions: ["json"] }],
      });

      if (!filePath) {
        return false;
      }

      await api.saveToFile(filePath, data);
      return true;
    } finally {
      exporting.value = false;
      await api.setIgnoreBlur(false);
    }
  }

  async function importData() {
    if (exporting.value || importing.value) {
      return false;
    }

    try {
      importing.value = true;
      await api.setIgnoreBlur(true);
      const filePath = await open({
        multiple: false,
        filters: [{ name: "JSON", extensions: ["json"] }],
      });

      if (!filePath || Array.isArray(filePath)) {
        return false;
      }

      const content = await api.readFile(filePath);
      await api.importData(content);
      await notifyDataChanged();
      return true;
    } finally {
      importing.value = false;
      await api.setIgnoreBlur(false);
    }
  }

  return {
    exporting,
    importing,
    exportData,
    importData,
  };
}
