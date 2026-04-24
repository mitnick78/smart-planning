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

interface CalendarDayProps {
  date: string;
  dayNumber: number;
  isToday: boolean;
  isCurrentMonth: boolean;
}

export default function CalendarDay({
  date,
  dayNumber,
  isToday,
  isCurrentMonth,
}: CalendarDayProps) {
  const [opened, { open, close }] = useDisclosure(false);

  const activeLabel = useAppStore(
    useShallow((s) => s.labels.find((l) => l.id === s.activeLabelId) ?? null),
  );
  const dayEntries = useAppStore(useShallow((s) => s.planningMap[date] ?? {}));

  const { assignLabel, clearSlot } = usePlanning();

  // Couleur dominante du jour — première période assignée
  const dominantColor =
    PERIOD_ORDER.map((p) => dayEntries[p]?.labelColor).find(Boolean) ?? null;

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
        onClick={() => {
          open();
        }}
        style={{
          background: dominantColor
            ? dominantColor + "15"
            : "var(--mantine-color-dark-6)",
          borderRadius: 6,
          padding: "4px 6px",
          minHeight: 64,
          border: isToday
            ? "1px solid var(--mantine-color-blue-5)"
            : dominantColor
              ? `1px solid ${dominantColor}44`
              : "1px solid var(--mantine-color-dark-4)",
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

        {PERIOD_ORDER.filter((p) => dayEntries[p])
          .slice(0, 2)
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

        {Object.keys(dayEntries).length > 2 && (
          <Text size="xs" c="dimmed" style={{ fontSize: 9 }}>
            +{Object.keys(dayEntries).length - 2} autres
          </Text>
        )}
      </Box>

      <Modal opened={opened} onClose={close} title={date} size="sm" centered>
        <Stack gap="xs">
          {PERIOD_ORDER.map((period) => {
            const entry = dayEntries[period];
            const color = entry?.labelColor ?? "#888";

            if (entry) {
              // occuped slot
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

            // empty slot
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
