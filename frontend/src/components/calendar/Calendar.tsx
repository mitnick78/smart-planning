import { useMemo } from "react";
import { Stack, Group, Text, ActionIcon, Button } from "@mantine/core";
import { IconChevronLeft, IconChevronRight } from "@tabler/icons-react";
import { usePlanning } from "@/hooks";
import CalendarDay from "./CalendarDay";

const DAYS_OF_WEEK = ["Lun", "Mar", "Mer", "Jeu", "Ven", "Sam", "Dim"];

const MONTH_NAMES = [
  "Janvier",
  "Février",
  "Mars",
  "Avril",
  "Mai",
  "Juin",
  "Juillet",
  "Août",
  "Septembre",
  "Octobre",
  "Novembre",
  "Décembre",
];

export default function Calendar() {
  const { currentYear, currentMonth, goToNextMonth, goToPrevMonth, goToToday } =
    usePlanning();

  const days = useMemo(() => {
    const firstDay = new Date(currentYear, currentMonth - 1, 1).getDay();
    const daysInMonth = new Date(currentYear, currentMonth, 0).getDate();
    const daysInPrevMonth = new Date(
      currentYear,
      currentMonth - 1,
      0,
    ).getDate();
    const offset = firstDay === 0 ? 6 : firstDay - 1;

    const result: { date: string; day: number; isCurrentMonth: boolean }[] = [];

    // days from previous month
    for (let i = offset - 1; i >= 0; i--) {
      const day = daysInPrevMonth - i;
      const month = currentMonth === 1 ? 12 : currentMonth - 1;
      const year = currentMonth === 1 ? currentYear - 1 : currentYear;
      result.push({
        date: `${year}-${String(month).padStart(2, "0")}-${String(day).padStart(2, "0")}`,
        day,
        isCurrentMonth: false,
      });
    }

    // days from current month
    for (let d = 1; d <= daysInMonth; d++) {
      result.push({
        date: `${currentYear}-${String(currentMonth).padStart(2, "0")}-${String(d).padStart(2, "0")}`,
        day: d,
        isCurrentMonth: true,
      });
    }

    // days from next month
    const remaining = 42 - result.length;
    for (let d = 1; d <= remaining; d++) {
      const month = currentMonth === 12 ? 1 : currentMonth + 1;
      const year = currentMonth === 12 ? currentYear + 1 : currentYear;
      result.push({
        date: `${year}-${String(month).padStart(2, "0")}-${String(d).padStart(2, "0")}`,
        day: d,
        isCurrentMonth: false,
      });
    }

    return result;
  }, [currentYear, currentMonth]);

  const todayStr = useMemo(() => {
    const now = new Date();
    return `${now.getFullYear()}-${String(now.getMonth() + 1).padStart(2, "0")}-${String(now.getDate()).padStart(2, "0")}`;
  }, []);

  return (
    <Stack
      p="md"
      gap="sm"
      style={{
        height: "calc(100vh - 48px)", // 48px = hauteur du header
        overflow: "hidden",
      }}
    >
      <Group justify="space-between">
        <Group gap="xs">
          <ActionIcon variant="subtle" onClick={goToPrevMonth}>
            <IconChevronLeft size={16} />
          </ActionIcon>
          <Text fw={500} size="md" w={160} ta="center">
            {MONTH_NAMES[currentMonth - 1]} {currentYear}
          </Text>
          <ActionIcon variant="subtle" onClick={goToNextMonth}>
            <IconChevronRight size={16} />
          </ActionIcon>
        </Group>
        <Button variant="subtle" size="xs" onClick={goToToday}>
          Aujourd'hui
        </Button>
      </Group>

      <div
        style={{
          display: "grid",
          gridTemplateColumns: "repeat(7, 1fr)",
          gridTemplateRows: "auto repeat(6, 1fr)",
          gap: 4,
          flex: 1,
          height: "100%",
          overflow: "hidden",
        }}
      >
        {DAYS_OF_WEEK.map((dow) => (
          <Text key={dow} size="xs" c="dimmed" ta="center" fw={500} py={4}>
            {dow}
          </Text>
        ))}

        {days.map(({ date, day, isCurrentMonth }) => (
          <CalendarDay
            key={date}
            date={date}
            dayNumber={day}
            isToday={date === todayStr}
            isCurrentMonth={isCurrentMonth}
          />
        ))}
      </div>
    </Stack>
  );
}
