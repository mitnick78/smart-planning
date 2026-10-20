import {
  Box,
  Text,
  Stack,
  Group,
  Modal,
  Tooltip,
  ColorSwatch,
  ActionIcon,
} from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { notifications } from "@mantine/notifications";
import { IconX } from "@tabler/icons-react";
import { useShallow } from "zustand/react/shallow";
import { useAppStore } from "@/store/appStore";
import { usePlanning } from "@/hooks";
import { PERIOD_EMOJI, PERIOD_ORDER } from "@/types";
import type { Period } from "@/types";

const HOLIDAY_COLOR = "#F9A825";
const HOLIDAY_BG = "#F9A82518";

interface CalendarDayProps {
  date: string;
  dayNumber: number;
  isToday: boolean;
  isCurrentMonth: boolean;
  holidayName?: string;
}

export default function CalendarDay({
  date,
  dayNumber,
  isToday,
  isCurrentMonth,
  holidayName,
}: CalendarDayProps) {
  const [opened, { open, close }] = useDisclosure(false);

  const activeLabel = useAppStore(
    useShallow((s) => s.labels.find((l) => l.id === s.activeLabelId) ?? null),
  );
  const dayEntries = useAppStore(useShallow((s) => s.planningMap[date] ?? {}));

  const { assignLabel, clearSlot } = usePlanning();

  const dominantColor =
    PERIOD_ORDER.map((p) => dayEntries[p]?.labelColor).find(Boolean) ?? null;

  const isHoliday = !!holidayName;
  const hasLabel = Object.keys(dayEntries).length > 0;

  // Background: priority label; otherwise yellow if on vacation; otherwise default
  const bgColor = dominantColor
    ? dominantColor + "15"
    : isHoliday
      ? HOLIDAY_BG
      : "var(--mantine-color-dark-6)";

  // Border: blue today, color label, yellow on holidays, defect
  const borderStyle = isToday
    ? "1px solid var(--mantine-color-blue-5)"
    : dominantColor
      ? `1px solid ${dominantColor}44`
      : isHoliday
        ? `1px solid ${HOLIDAY_COLOR}55`
        : "1px solid var(--mantine-color-dark-4)";

  // Number of visible labels: reduced to 1 if on vacation to make room
  const maxVisibleLabels = isHoliday ? 1 : 2;

  const handleSlotClick = async (period: Period) => {
    if (!activeLabel?.id) {
      notifications.show({
        message: "Sélectionne un label dans le panneau gauche",
        color: "yellow",
      });
      return;
    }
    try {
      await assignLabel(
        date,
        period,
        activeLabel.id,
        activeLabel.name,
        activeLabel.colorHex,
      );
      notifications.show({
        message: `${PERIOD_EMOJI[period]} ${activeLabel.name} assigné`,
        color: "green",
      });
    } catch {
      notifications.show({ message: "Erreur assignation", color: "red" });
    }
  };

  const handleClearSlot = async (period: Period, e: React.MouseEvent) => {
    e.stopPropagation();
    try {
      await clearSlot(date, period);
      notifications.show({ message: "Créneau vidé", color: "green" });
    } catch {
      notifications.show({ message: "Erreur suppression", color: "red" });
    }
  };

  return (
    <>
      <Box
        onClick={open}
        style={{
          background: bgColor,
          borderRadius: 6,
          padding: "4px 6px",
          minHeight: 64,
          border: borderStyle,
          cursor: "pointer",
          opacity: isCurrentMonth ? 1 : 0.4,
          display: "flex",
          flexDirection: "column",
          gap: 2,
          transition: "background 0.15s",
        }}
      >
        <Text
          size="xs"
          fw={isToday ? 700 : 400}
          c={isToday ? "blue" : "dimmed"}
        >
          {dayNumber}
        </Text>

        {/* Labels users */}
        {PERIOD_ORDER.filter((p) => dayEntries[p])
          .slice(0, maxVisibleLabels)
          .map((period) => {
            const entry = dayEntries[period]!;
            const color = entry.labelColor ?? "#888";
            return (
              <Box
                key={period}
                style={{
                  background: color + "33",
                  borderLeft: `2px solid ${color}`,
                  borderRadius: 3,
                  padding: "1px 4px",
                  fontSize: 9,
                  color,
                  fontWeight: 500,
                  whiteSpace: "nowrap",
                  overflow: "hidden",
                  textOverflow: "ellipsis",
                }}
              >
                {PERIOD_EMOJI[period as Period]} {entry.labelName}
              </Box>
            );
          })}

        {/* School Break Badge */}
        {isHoliday && (
          <Box
            style={{
              background: HOLIDAY_BG,
              borderLeft: `2px solid ${HOLIDAY_COLOR}`,
              borderRadius: 3,
              padding: "1px 4px",
              fontSize: 9,
              color: HOLIDAY_COLOR,
              fontWeight: 500,
              whiteSpace: "nowrap",
              overflow: "hidden",
              textOverflow: "ellipsis",
            }}
          >
            🏖️ {hasLabel ? "Vac." : "Vacances"}
          </Box>
        )}

        {Object.keys(dayEntries).length > maxVisibleLabels && (
          <Text size="xs" c="dimmed" style={{ fontSize: 9 }}>
            +{Object.keys(dayEntries).length - maxVisibleLabels} autres
          </Text>
        )}
      </Box>

      <Modal opened={opened} onClose={close} title={date} size="sm" centered>
        <Stack gap="xs">
          {/* School Break Banner at the top of the modal */}
          {isHoliday && (
            <Box
              p="xs"
              style={{
                background: HOLIDAY_BG,
                border: `1px solid ${HOLIDAY_COLOR}44`,
                borderRadius: 8,
              }}
            >
              <Text size="sm" fw={500} style={{ color: HOLIDAY_COLOR }}>
                🏖️ {holidayName}
              </Text>
            </Box>
          )}

          {PERIOD_ORDER.map((period) => {
            const entry = dayEntries[period];
            const color = entry?.labelColor ?? "#888";

            if (entry) {
              return (
                <Group
                  key={period}
                  justify="space-between"
                  p="xs"
                  style={{
                    background: color + "15",
                    borderRadius: 8,
                    border: `1px solid ${color}44`,
                  }}
                >
                  <Group gap="xs">
                    <Text size="sm" c="dimmed" w={90}>
                      {PERIOD_EMOJI[period]} {period}
                    </Text>
                    <Box
                      style={{
                        background: color + "33",
                        borderLeft: `2px solid ${color}`,
                        borderRadius: 4,
                        padding: "2px 8px",
                        fontSize: 12,
                        color,
                        fontWeight: 500,
                      }}
                    >
                      {entry.labelName}
                    </Box>
                  </Group>
                  <Tooltip label="Vider ce créneau">
                    <ActionIcon
                      size="sm"
                      variant="subtle"
                      color="red"
                      onClick={(e) => handleClearSlot(period, e)}
                    >
                      <IconX size={12} />
                    </ActionIcon>
                  </Tooltip>
                </Group>
              );
            }

            return (
              <Group
                key={period}
                p="xs"
                onClick={() => handleSlotClick(period)}
                style={{
                  border: "1px dashed var(--mantine-color-dark-4)",
                  borderRadius: 8,
                  cursor: activeLabel ? "pointer" : "default",
                  opacity: activeLabel ? 1 : 0.5,
                  transition: "background 0.15s",
                }}
                onMouseEnter={(e) => {
                  if (activeLabel) {
                    e.currentTarget.style.background =
                      "var(--mantine-color-dark-6)";
                  }
                }}
                onMouseLeave={(e) => {
                  e.currentTarget.style.background = "transparent";
                }}
              >
                <Text size="sm" c="dimmed" w={90}>
                  {PERIOD_EMOJI[period]} {period}
                </Text>
                {activeLabel ? (
                  <Group gap={6}>
                    <ColorSwatch color={activeLabel.colorHex} size={10} />
                    <Text size="sm" c="dimmed">
                      + {activeLabel.name}
                    </Text>
                  </Group>
                ) : (
                  <Text size="sm" c="dimmed">
                    Sélectionne un label
                  </Text>
                )}
              </Group>
            );
          })}
        </Stack>
      </Modal>
    </>
  );
}
