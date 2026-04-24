import { invoke } from "@tauri-apps/api/core";
import type { Label, LabelCreateinput, LabelUpdateInput } from "@/types";

// GET ALL LABELS
export const getLabels = async (): Promise<Label[]> => {
  try {
    const labels = await invoke<Label[]>("get_labels");
    return labels;
  } catch (error) {
    console.error("Error fetching labels:", error);
    throw error;
  }
};

// CREATE A NEW LABEL
export const createLabel = async (input: LabelCreateinput): Promise<Label> => {
  try {
    const newLabel = await invoke<Label>("create_label", {
      name: input.name,
      colorHex: input.colorHex,
    });
    return newLabel;
  } catch (error) {
    console.error("Error creating label:", error);
    throw error;
  }
};

// UPDATE AN EXISTING LABEL
export const updateLabel = async (input: LabelUpdateInput): Promise<Label> => {
  try {
    const updatedLabel = await invoke<Label>("update_label", {
      name: input.name,
      colorHex: input.colorHex,
    });
    return updatedLabel;
  } catch (error) {
    console.error("Error updating label:", error);
    throw error;
  }
};

// DELETE A LABEL
export const deleteLabel = async (
  id: number,
  deleteEntries: boolean = false,
): Promise<void> => {
  try {
    await invoke("delete_label", { id, deleteEntries });
  } catch (error) {
    console.error("Error deleting label:", error);
    throw error;
  }
};
