import { invoke } from "@tauri-apps/api/core";
import type {
  PlanningEntry,
  UpsertPlanningInput,
  StatEntry,
  Period,
} from "@/types";

export const getPlanningByMonth = async (
  year: number, // ← year en premier
  month: number,
): Promise<PlanningEntry[]> => {
  try {
    return await invoke<PlanningEntry[]>("get_planning_by_month", {
      year,
      month,
    });
  } catch (error) {
    console.error("Error fetching planning:", error);
    throw error;
  }
};

export const upsertPlanning = async (
  input: UpsertPlanningInput,
): Promise<number> => {
  try {
    return await invoke<number>("upsert_planning", {
      date: input.date,
      period: input.period,
      labelId: input.labelId,
      note: input.note ?? null,
    });
  } catch (error) {
    console.error("Error upserting:", error);
    throw error;
  }
};

export const deletePlanning = async (
  date: string,
  period: Period,
): Promise<void> => {
  try {
    await invoke("delete_planning", { date, period });
  } catch (error) {
    console.error("Error deleting:", error);
    throw error;
  }
};

export const getStatsByMonth = async (
  year: number,
  month: number,
): Promise<StatEntry[]> => {
  try {
    return await invoke<StatEntry[]>("get_stats_by_month", { year, month });
  } catch (error) {
    console.error("Error fetching stats:", error);
    throw error;
  }
};

export const exportToExcel = (
  year: number,
  months: number[],
): Promise<string> => invoke<string>("export_to_excel", { year, months });
