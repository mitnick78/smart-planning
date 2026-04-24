export interface StatEntry {
  labelName: string;
  labelColor: string;
  period: string;
  count: number;
}

// stats for a single label, grouped by period (e.g., day, week, month)
export interface LabelStats {
  labelName: string;
  labelColor: string;
  periods: {
    period: string;
    count: number;
  }[];
  total: number;
}
