import { listen, emit } from "@tauri-apps/api/event";

const DATA_CHANGED_EVENT = "data-changed";

export function useEditorSync() {
  async function notifyDataChanged() {
    await emit(DATA_CHANGED_EVENT);
  }

  async function onDataChanged(handler: () => void | Promise<void>) {
    return listen(DATA_CHANGED_EVENT, async () => {
      await handler();
    });
  }

  return {
    notifyDataChanged,
    onDataChanged,
  };
}
