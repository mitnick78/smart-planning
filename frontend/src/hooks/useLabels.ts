import { useCallback, useEffect } from "react";
import { useAppStore } from "@/store/appStore";
import * as labelApi from "@/api/labelApi";
import type { LabelCreateinput, LabelUpdateInput } from "@/types";

export const useLabels = () => {
  // Store state
  const labels = useAppStore((s) => s.labels);
  const activeLabelId = useAppStore((s) => s.activeLabelId);
  const isLoading = useAppStore((s) => s.isLoading);
  const error = useAppStore((s) => s.error);

  // Actions du store
  const setLabels = useAppStore((s) => s.setLabels);
  const addLabel = useAppStore((s) => s.addLabel);
  const updateLabelStore = useAppStore((s) => s.updateLabel);
  const removeLabelStore = useAppStore((s) => s.removeLabel);
  const setActiveLabelId = useAppStore((s) => s.setActiveLabelId);
  const setLoading = useAppStore((s) => s.setLoading);
  const setError = useAppStore((s) => s.setError);

  // memoized  for useEffect dependency
  const loadLabels = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const data = await labelApi.getLabels();
      setLabels(data);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Erreur inconnue");
    } finally {
      setLoading(false);
    }
  }, [setError, setLabels, setLoading]);

  // load initial labels on mount
  useEffect(() => {
    loadLabels();
  }, [loadLabels]);

  // Actions
  const createLabel = useCallback(
    async (input: LabelCreateinput) => {
      setLoading(true);
      setError(null);
      try {
        const newLabel = await labelApi.createLabel(input);
        addLabel(newLabel);
        // select the newly created label by default
        setActiveLabelId(newLabel.id);
        return newLabel;
      } catch (err) {
        setError(err instanceof Error ? err.message : "Erreur création");
        throw err;
      } finally {
        setLoading(false);
      }
    },
    [addLabel, setActiveLabelId, setError, setLoading],
  );

  const updateLabel = useCallback(
    async (input: LabelUpdateInput) => {
      setLoading(true);
      setError(null);
      try {
        await labelApi.updateLabel(input);
        const existingLabel = labels.find((l) => l.id === input.id);
        // uodate local store with new values, but keep createdAt if it exists (since API doesn't return it)
        updateLabelStore({
          id: input.id,
          name: input.name,
          colorHex: input.colorHex,
          createdAt: existingLabel?.createdAt ?? new Date().toISOString(), // garde la date de création
        });
      } catch (err) {
        setError(err instanceof Error ? err.message : "Erreur mise à jour");
        throw err;
      } finally {
        setLoading(false);
      }
    },
    [labels, setError, setLoading, updateLabelStore],
  );

  const deleteLabel = useCallback(
    async (id: number, deleteEntries: boolean = false) => {
      setLoading(true);
      setError(null);
      try {
        await labelApi.deleteLabel(id, deleteEntries);
        removeLabelStore(id);
      } catch (err) {
        setError(err instanceof Error ? err.message : "Erreur suppression");
        throw err;
      } finally {
        setLoading(false);
      }
    },
    [removeLabelStore, setError, setLoading],
  );

  // activeLabel is derived from activeLabelId and labels list
  const activeLabel = labels.find((l) => l.id === activeLabelId) ?? null;

  return {
    // State
    labels,
    activeLabel,
    activeLabelId,
    isLoading,
    error,

    // Actions
    loadLabels,
    createLabel,
    updateLabel,
    deleteLabel,
    setActiveLabelId,
  };
};
