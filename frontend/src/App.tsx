import { AppShell, Group, Text } from "@mantine/core";
import { IconCalendar } from "@tabler/icons-react";
import LabelsPanel from "@/components/labels/LabelsPanel";
import Calendar from "@/components/calendar/Calendar";
import StatsPanel from "@/components/stats/StatsPanel";

function App() {
  return (
    <AppShell
      header={{ height: 48 }}
      navbar={{ width: 220, breakpoint: "sm" }}
      aside={{ width: 200, breakpoint: "sm" }}
      padding={0}
    >
      <AppShell.Header>
        <Group h="100%" px="md">
          <IconCalendar size={18} />
          <Text fw={500} size="sm">
            Smart Planning Pro
          </Text>
        </Group>
      </AppShell.Header>

      <AppShell.Navbar p="sm">
        <LabelsPanel />
      </AppShell.Navbar>

      <AppShell.Main style={{ height: "100vh", overflow: "hidden" }}>
        <Calendar />
      </AppShell.Main>

      <AppShell.Aside p="sm">
        <StatsPanel />
      </AppShell.Aside>
    </AppShell>
  );
}

export default App;
