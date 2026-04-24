import { useCallback, useEffect } from "react";
import { useAppStore } from "@/store/appStore";
import * as planningApi from "@/api/planningApi";
import { getStatsByMonth } from "@/api/planningApi";
import type { Period, PlanningEntry } from "@/types";

export const usePlanning = () => {
  // ── Lecture du store ────────────────────────────────────────
  const entries = useAppStore((s) => s.entries);
  const planningMap = useAppStore((s) => s.planningMap);
  const currentYear = useAppStore((s) => s.currentYear);
  const currentMonth = useAppStore((s) => s.currentMonth);
  const isLoading = useAppStore((s) => s.isLoading);
  const error = useAppStore((s) => s.error);

  // ── Actions du store ────────────────────────────────────────
  const setEntries = useAppStore((s) => s.setEntries);
  const upsertEntry = useAppStore((s) => s.upsertEntry);
  const removeEntry = useAppStore((s) => s.removeEntry);
  const setCurrentMonth = useAppStore((s) => s.setCurrentMonth);
  const setStats = useAppStore((s) => s.setStats);
  const setLoading = useAppStore((s) => s.setLoading);
  const setError = useAppStore((s) => s.setError);

  const loadMonth = useCallback(
    async (year: number, month: number) => {
      setLoading(true);
      setError(null);
      try {
        // Load planning et statsin parallel
        const [data, stats] = await Promise.all([
          planningApi.getPlanningByMonth(year, month),
          getStatsByMonth(year, month),
        ]);
        setEntries(data);
        setStats(stats);
      } catch (err) {
        setError(err instanceof Error ? err.message : "Erreur chargement");
      } finally {
        setLoading(false);
      }
    },
    [setEntries, setStats, setLoading, setError],
  );

  // loading initial current month
  useEffect(() => {
    loadMonth(currentYear, currentMonth);
  }, [currentYear, currentMonth]);

  // Actions
  // navigate between months
  const goToNextMonth = useCallback(() => {
    const { currentYear, currentMonth } = useAppStore.getState();
    if (currentMonth === 12) {
      setCurrentMonth(currentYear + 1, 1);
    } else {
      setCurrentMonth(currentYear, currentMonth + 1);
    }
  }, [setCurrentMonth]);

  const goToPrevMonth = useCallback(() => {
    const { currentYear, currentMonth } = useAppStore.getState();
    if (currentMonth === 1) {
      setCurrentMonth(currentYear - 1, 12);
    } else {
      setCurrentMonth(currentYear, currentMonth - 1);
    }
  }, [setCurrentMonth]);

  const goToToday = useCallback(() => {
    const now = new Date();
    setCurrentMonth(now.getFullYear(), now.getMonth() + 1);
  }, [setCurrentMonth]);

  // assign a label to a slot
  const assignLabel = useCallback(
    async (
      date: string,
      period: Period,
      labelId: number,
      labelName: string,
      labelColor: string,
      note?: string,
    ) => {
      setError(null);
      try {
        await planningApi.upsertPlanning({ date, period, labelId, note });

        // update local state immediately without waiting for the next load
        const newEntry: PlanningEntry = {
          id: null,
          date,
          period,
          labelId,
          note: note ?? null,
          labelName,
          labelColor,
          createdAt: null,
          updatedAt: null,
        };
        upsertEntry(newEntry);

        // load stats immediately after update
        const stats = await getStatsByMonth(currentYear, currentMonth);
        setStats(stats);
      } catch (err) {
        setError(err instanceof Error ? err.message : "Erreur assignation");
        throw err;
      }
    },
    [currentYear, currentMonth, upsertEntry, setStats, setError],
  );

  // delete a slot
  const clearSlot = useCallback(
    async (date: string, period: Period) => {
      setError(null);
      try {
        await planningApi.deletePlanning(date, period);
        removeEntry(date, period);

        // load stats immediately after update
        const stats = await getStatsByMonth(currentYear, currentMonth);
        setStats(stats);
      } catch (err) {
        setError(err instanceof Error ? err.message : "Erreur suppression");
        throw err;
      }
    },
    [currentYear, currentMonth, removeEntry, setStats, setError],
  );

  // get entry for a specific date and period
  const getEntry = useCallback(
    (date: string, period: Period) => {
      return planningMap[date]?.[period] ?? null;
    },
    [planningMap],
  );

  return {
    // State
    entries,
    planningMap,
    currentYear,
    currentMonth,
    isLoading,
    error,

    // Actions
    loadMonth,
    goToNextMonth,
    goToPrevMonth,
    goToToday,
    assignLabel,
    clearSlot,
    getEntry,
  };
};
