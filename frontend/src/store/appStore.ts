import { create } from "zustand";
import type {
  Label,
  PlanningEntry,
  StatEntry,
  Period,
  MonthPlanningMap,
} from "@/types";

interface AppState {
  // labels
  labels: Label[];
  activeLabelId: number | null; // label selected in the UI, not necessarily the one in the current entry

  // Calendar
  currentYear: number;
  currentMonth: number;
  planningMap: MonthPlanningMap; // Map for quick access to entries by date and period
  entries: PlanningEntry[]; // list of entries for the current month, used to build the planningMap

  // Stats
  stats: StatEntry[];

  // UI
  isLoading: boolean;
  error: string | null;

  // Actions Labels
  setLabels: (labels: Label[]) => void;
  addLabel: (label: Label) => void;
  updateLabel: (label: Label) => void;
  removeLabel: (id: number) => void;
  setActiveLabelId: (id: number | null) => void;

  // Calendar Actions
  setCurrentMonth: (year: number, month: number) => void;
  setEntries: (entries: PlanningEntry[]) => void;
  upsertEntry: (entry: PlanningEntry) => void;
  removeEntry: (date: string, period: Period) => void;

  // Actions Stats
  setStats: (stats: StatEntry[]) => void;

  // Actions UI
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
}

// Helper
// convert PlanningEntry[] in MonthPlanningMap for quick access to entries by date and period
const buildPlanningMap = (entries: PlanningEntry[]): MonthPlanningMap => {
  return entries.reduce((map, entry) => {
    if (!map[entry.date]) {
      map[entry.date] = {};
    }
    map[entry.date][entry.period] = entry;
    return map;
  }, {} as MonthPlanningMap);
};

// Store
export const useAppStore = create<AppState>((set, get) => ({
  // state initial
  labels: [],
  activeLabelId: null,

  currentYear: new Date().getFullYear(),
  currentMonth: new Date().getMonth() + 1, // getMonth() = 0-11 → +1
  planningMap: {},
  entries: [],

  stats: [],

  isLoading: false,
  error: null,

  // Actions Labels
  setLabels: (labels) => set({ labels }),

  addLabel: (label) => set((state) => ({ labels: [...state.labels, label] })),

  updateLabel: (label) =>
    set((state) => ({
      labels: state.labels.map((l) => (l.id === label.id ? label : l)),
    })),

  removeLabel: (id) =>
    set((state) => ({
      labels: state.labels.filter((l) => l.id !== id),
      // if the removed label was active, we can either set activeLabelId to null or keep it (depends on desired UX)
      activeLabelId: state.activeLabelId === id ? null : state.activeLabelId,
    })),

  setActiveLabelId: (id) => set({ activeLabelId: id }),

  // Calendar Actions
  setCurrentMonth: (year, month) =>
    set({ currentYear: year, currentMonth: month }),

  setEntries: (entries) =>
    set({
      entries,
      planningMap: buildPlanningMap(entries),
    }),

  upsertEntry: (entry) => {
    const state = get();
    // if an entry already exists for the same date and period, we replace it, otherwise we add it
    const exists = state.entries.some(
      (e) => e.date === entry.date && e.period === entry.period,
    );
    const newEntries = exists
      ? state.entries.map((e) =>
          e.date === entry.date && e.period === entry.period ? entry : e,
        )
      : [...state.entries, entry];

    set({
      entries: newEntries,
      planningMap: buildPlanningMap(newEntries),
    });
  },

  removeEntry: (date, period) => {
    const newEntries = get().entries.filter(
      (e) => !(e.date === date && e.period === period),
    );
    set({
      entries: newEntries,
      planningMap: buildPlanningMap(newEntries),
    });
  },

  // Actions Stats
  setStats: (stats) => set({ stats }),

  // Actions UI
  setLoading: (isLoading) => set({ isLoading }),
  setError: (error) => set({ error }),
}));

// backward compatibility
export const useActiveLabel = () =>
  useAppStore(
    (state) => state.labels.find((l) => l.id === state.activeLabelId) ?? null,
  );

// back to planningMap for quick access to entries by date and period
export const useDayEntries = (date: string) =>
  useAppStore((state) => state.planningMap[date] ?? {});

// Returns a specific entry (date + period)
export const useEntry = (date: string, period: Period) =>
  useAppStore((state) => state.planningMap[date]?.[period] ?? null);
