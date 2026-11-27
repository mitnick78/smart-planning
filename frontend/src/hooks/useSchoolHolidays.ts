import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { notifications } from "@mantine/notifications";
import { toErrorMessage } from "@/api/errors";

const notifyError = (err: unknown, fallback: string) =>
  notifications.show({ message: toErrorMessage(err, fallback), color: "red" });

export interface SchoolHoliday {
  date: string;
  zone: string;
  name: string;
}

export function useSchoolHolidays(year: number, month: number) {
  const [zone, setZoneState] = useState("C");
  const [holidays, setHolidays] = useState<Map<string, string>>(new Map());
  const [syncing, setSyncing] = useState(false);

  // Load the saved area at startup
  useEffect(() => {
    invoke<string | null>("get_setting", { key: "school_zone" })
      .then((v) => {
        if (v) setZoneState(v);
      })
      .catch((err) => notifyError(err, "Zone scolaire non chargée"));
  }, []);

  // Reload from SQLite whenever the month or region changes
  useEffect(() => {
    invoke<SchoolHoliday[]>("get_school_holidays_by_month", {
      year,
      month,
      zone,
    })
      .then((data) => {
        setHolidays(new Map(data.map((h) => [h.date, h.name])));
      })
      .catch((err) => notifyError(err, "Vacances scolaires non chargées"));
  }, [year, month, zone]);

  const setZone = (z: string) => {
    setZoneState(z);
    invoke("set_setting", { key: "school_zone", value: z }).catch((err) =>
      notifyError(err, "Zone scolaire non enregistrée"),
    );
  };

  const syncHolidays = async () => {
    setSyncing(true);
    try {
      const count = await invoke<number>("fetch_school_holidays", {
        zone,
        year,
      });
      // Refresh from SQLite after sync
      const data = await invoke<SchoolHoliday[]>(
        "get_school_holidays_by_month",
        { year, month, zone },
      );
      setHolidays(new Map(data.map((h) => [h.date, h.name])));
      notifications.show({
        message: `${count} jours de vacances synchronisés (Zone ${zone})`,
        color: "yellow",
      });
    } catch (e) {
      notifyError(e, "Erreur de synchronisation des vacances");
    } finally {
      setSyncing(false);
    }
  };

  return { zone, setZone, holidays, syncing, syncHolidays };
}
