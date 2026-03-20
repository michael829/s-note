import { ref } from "vue";
import { enable, disable, isEnabled } from "@tauri-apps/plugin-autostart";

const enabled = ref(false);

export function useAutoStart() {
  async function load() {
    try {
      enabled.value = await isEnabled();
    } catch (_) {
      enabled.value = false;
    }
  }

  async function toggle() {
    if (enabled.value) {
      await disable();
      enabled.value = false;
      return false;
    }

    await enable();
    enabled.value = true;
    return true;
  }

  return {
    enabled,
    load,
    toggle,
  };
}
