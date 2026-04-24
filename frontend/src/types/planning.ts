export type Period = "matin" | "après-midi" | "soir" | "journée";

// emoji by period for display in the calendar
export const PERIOD_EMOJI: Record<Period, string> = {
  matin: "🌅",
  "après-midi": "☀️",
  soir: "🌙",
  journée: "📅",
};

// Order of display in the calendar
export const PERIOD_ORDER: Period[] = [
  "matin",
  "après-midi",
  "soir",
  "journée",
];

export interface PlanningEntry {
  id: number | null;
  date: string;
  period: Period;
  labelId: number;
  note: string | null;
  labelName: string | null;
  labelColor: string | null;
  createdAt: string | null;
  updatedAt: string | null;
}

export type UpsertPlanningInput = {
  date: string;
  period: Period;
  labelId: number;
  note?: string | null;
};

export type MonthPlanningMap = Record<
  string,
  Partial<Record<Period, PlanningEntry>>
>;
