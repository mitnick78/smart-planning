import { useAppStore } from "@/store/appStore";
import type { LabelStats } from "@/types";

export const useStats = () => {
  // read stats and current period from the store
  const stats = useAppStore((s) => s.stats);
  const currentYear = useAppStore((s) => s.currentYear);
  const currentMonth = useAppStore((s) => s.currentMonth);

  // Group stats by label and calculate totals
  const groupedStats: LabelStats[] = stats.reduce((acc, stat) => {
    const existing = acc.find((s) => s.labelName === stat.labelName);
    if (existing) {
      existing.periods.push({ period: stat.period, count: stat.count });
      existing.total += stat.count;
    } else {
      acc.push({
        labelName: stat.labelName,
        labelColor: stat.labelColor,
        periods: [{ period: stat.period, count: stat.count }],
        total: stat.count,
      });
    }
    return acc;
  }, [] as LabelStats[]);

  const totalAssigned = stats.reduce((sum, s) => sum + s.count, 0);

  return {
    stats,
    groupedStats,
    totalAssigned,
    currentYear,
    currentMonth,
  };
};
