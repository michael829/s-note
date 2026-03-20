import { beforeEach, describe, expect, it, vi } from "vitest";

const enable = vi.fn();
const disable = vi.fn();
const isEnabled = vi.fn();

vi.mock("@tauri-apps/plugin-autostart", () => ({
  enable,
  disable,
  isEnabled,
}));

describe("useAutoStart", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("loads enabled state", async () => {
    isEnabled.mockResolvedValue(true);
    const { useAutoStart } = await import("../useAutoStart");

    const autoStart = useAutoStart();
    await autoStart.load();

    expect(autoStart.enabled.value).toBe(true);
  });

  it("toggles from disabled to enabled", async () => {
    const { useAutoStart } = await import("../useAutoStart");
    const autoStart = useAutoStart();
    autoStart.enabled.value = false;

    const result = await autoStart.toggle();

    expect(enable).toHaveBeenCalled();
    expect(result).toBe(true);
    expect(autoStart.enabled.value).toBe(true);
  });

  it("toggles from enabled to disabled", async () => {
    const { useAutoStart } = await import("../useAutoStart");
    const autoStart = useAutoStart();
    autoStart.enabled.value = true;

    const result = await autoStart.toggle();

    expect(disable).toHaveBeenCalled();
    expect(result).toBe(false);
    expect(autoStart.enabled.value).toBe(false);
  });
});
