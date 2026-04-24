import {
  Stack,
  Text,
  Group,
  ColorSwatch,
  Box,
  Progress,
  Divider,
  Button,
} from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { IconDownload } from "@tabler/icons-react";
import { useStats } from "@/hooks";
import { PERIOD_EMOJI } from "@/types";
import type { Period } from "@/types";
import ExportModal from "./ExportModal";

export default function StatsPanel() {
  const { groupedStats, totalAssigned, currentYear, currentMonth } = useStats();
  const [exportOpened, { open: openExport, close: closeExport }] =
    useDisclosure(false);

  return (
    <>
      <Stack gap="xs" h="100%">
        <Text size="xs" fw={500} c="dimmed" tt="uppercase" mb={4}>
          Statistiques
        </Text>
        <Box
          p="xs"
          style={{
            background: "var(--mantine-color-dark-6)",
            borderRadius: 8,
            border: "1px solid var(--mantine-color-dark-4)",
          }}
        >
          <Text size="xs" c="dimmed">
            Total assigné
          </Text>
          <Text size="xl" fw={700}>
            {totalAssigned}
          </Text>
          <Text size="xs" c="dimmed">
            créneaux ce mois
          </Text>
        </Box>

        <Divider />

        <Stack gap="sm" style={{ flex: 1, overflowY: "auto" }}>
          {groupedStats.length === 0 && (
            <Text size="xs" c="dimmed" ta="center" mt="xl">
              Aucune donnée ce mois
            </Text>
          )}

          {groupedStats.map((stat) => (
            <Box
              key={stat.labelName}
              p="xs"
              style={{
                background: "var(--mantine-color-dark-6)",
                borderRadius: 8,
                border: "1px solid var(--mantine-color-dark-4)",
              }}
            >
              <Group justify="space-between" mb={6}>
                <Group gap="xs">
                  <ColorSwatch color={stat.labelColor} size={10} />
                  <Text size="xs" fw={500}>
                    {stat.labelName}
                  </Text>
                </Group>
                <Text size="xs" fw={700} c="dimmed">
                  {stat.total}j
                </Text>
              </Group>

              <Stack gap={2}>
                {stat.periods.map(({ period, count }) => (
                  <Group key={period} justify="space-between">
                    <Text size="xs" c="dimmed">
                      {PERIOD_EMOJI[period as Period] ?? "📅"} {period}
                    </Text>
                    <Text size="xs">{count}j</Text>
                  </Group>
                ))}
              </Stack>

              <Progress
                mt={6}
                value={
                  totalAssigned > 0 ? (stat.total / totalAssigned) * 100 : 0
                }
                color={stat.labelColor}
                size="xs"
              />
            </Box>
          ))}
        </Stack>

        <Divider />

        <Button
          variant="subtle"
          leftSection={<IconDownload size={14} />}
          size="xs"
          onClick={openExport}
          fullWidth
        >
          Exporter Excel
        </Button>
      </Stack>

      <ExportModal
        opened={exportOpened}
        onClose={closeExport}
        currentYear={currentYear}
        currentMonth={currentMonth}
      />
    </>
  );
}
