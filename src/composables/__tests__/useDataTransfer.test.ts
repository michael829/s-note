import { beforeEach, describe, expect, it, vi } from "vitest";

const open = vi.fn();
const save = vi.fn();
const notifyDataChanged = vi.fn();
const api = {
  setIgnoreBlur: vi.fn(),
  exportData: vi.fn(),
  saveToFile: vi.fn(),
  readFile: vi.fn(),
  importData: vi.fn(),
};

vi.mock("@tauri-apps/plugin-dialog", () => ({
  open,
  save,
}));

vi.mock("../useApi", () => ({
  useApi: () => api,
}));

vi.mock("../useEditorSync", () => ({
  useEditorSync: () => ({
    notifyDataChanged,
  }),
}));

describe("useDataTransfer", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("exports data to selected file", async () => {
    save.mockResolvedValue("/tmp/out.json");
    api.exportData.mockResolvedValue('{"ok":true}');
    const { useDataTransfer } = await import("../useDataTransfer");

    const transfer = useDataTransfer();
    const result = await transfer.exportData();

    expect(result).toBe(true);
    expect(api.setIgnoreBlur).toHaveBeenNthCalledWith(1, true);
    expect(api.saveToFile).toHaveBeenCalledWith("/tmp/out.json", '{"ok":true}');
    expect(api.setIgnoreBlur).toHaveBeenLastCalledWith(false);
  });

  it("imports data from selected file and emits sync event", async () => {
    open.mockResolvedValue("/tmp/in.json");
    api.readFile.mockResolvedValue('{"ok":true}');
    const { useDataTransfer } = await import("../useDataTransfer");

    const transfer = useDataTransfer();
    const result = await transfer.importData();

    expect(result).toBe(true);
    expect(api.importData).toHaveBeenCalledWith('{"ok":true}');
    expect(notifyDataChanged).toHaveBeenCalled();
    expect(api.setIgnoreBlur).toHaveBeenLastCalledWith(false);
  });
});
