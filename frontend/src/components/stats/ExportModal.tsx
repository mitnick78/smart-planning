// ============================================================
//  frontend/src/components/stats/ExportModal.tsx
//  Export par plage de dates (multi-années)
// ============================================================

import { useState } from "react";
import {
  Modal,
  Button,
  Group,
  Stack,
  Select,
  Text,
  Alert,
  Divider,
} from "@mantine/core";
import {
  IconFileSpreadsheet,
  IconFileTypePdf,
  IconAlertCircle,
} from "@tabler/icons-react";
import { invoke } from "@tauri-apps/api/core";
import { useAppStore } from "@/store/appStore";

interface ExportModalProps {
  opened: boolean;
  onClose: () => void;
}

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

// Génère les options mois
const MONTH_OPTIONS = MONTH_NAMES.map((name, i) => ({
  value: String(i + 1),
  label: name,
}));

// Génère les années (5 ans en arrière, 2 ans en avant)
function getYearOptions() {
  const current = new Date().getFullYear();
  const years = [];
  for (let y = current - 5; y <= current + 2; y++) {
    years.push({ value: String(y), label: String(y) });
  }
  return years;
}

const YEAR_OPTIONS = getYearOptions();

export default function ExportModal({ opened, onClose }: ExportModalProps) {
  const currentYear = useAppStore((s) => s.currentYear);
  const currentMonth = useAppStore((s) => s.currentMonth);

  // Plage par défaut : mois courant → mois courant
  const [startYear, setStartYear] = useState(String(currentYear));
  const [startMonth, setStartMonth] = useState(String(currentMonth));
  const [endYear, setEndYear] = useState(String(currentYear));
  const [endMonth, setEndMonth] = useState(String(currentMonth));

  const [loadingExcel, setLoadingExcel] = useState(false);
  const [loadingPdf, setLoadingPdf] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);

  // Vérifie que la plage est valide
  const isRangeValid = () => {
    const start = parseInt(startYear) * 12 + parseInt(startMonth);
    const end = parseInt(endYear) * 12 + parseInt(endMonth);
    return start <= end;
  };

  // Nombre de mois dans la plage
  const nbMonths = () => {
    const start = parseInt(startYear) * 12 + parseInt(startMonth);
    const end = parseInt(endYear) * 12 + parseInt(endMonth);
    return Math.max(0, end - start + 1);
  };

  // Label du fichier généré
  const fileLabel = () => {
    const sm = MONTH_NAMES[parseInt(startMonth) - 1]
      .substring(0, 3)
      .toLowerCase();
    const em = MONTH_NAMES[parseInt(endMonth) - 1]
      .substring(0, 3)
      .toLowerCase();
    return `planning_${sm}${startYear}_${em}${endYear}`;
  };

  const handleExportExcel = async () => {
    if (!isRangeValid()) return;
    setLoadingExcel(true);
    setError(null);
    setSuccess(null);
    try {
      await invoke("export_to_excel", {
        startYear: parseInt(startYear),
        startMonth: parseInt(startMonth),
        endYear: parseInt(endYear),
        endMonth: parseInt(endMonth),
      });
      setSuccess(`Excel exporté : ${fileLabel()}.xlsx`);
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setLoadingExcel(false);
    }
  };

  const handleExportPdf = async () => {
    if (!isRangeValid()) return;
    setLoadingPdf(true);
    setError(null);
    setSuccess(null);
    try {
      await invoke("export_to_pdf", {
        startYear: parseInt(startYear),
        startMonth: parseInt(startMonth),
        endYear: parseInt(endYear),
        endMonth: parseInt(endMonth),
      });
      setSuccess(`PDF exporté : ${fileLabel()}.pdf`);
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setLoadingPdf(false);
    }
  };

  const handleClose = () => {
    setError(null);
    setSuccess(null);
    onClose();
  };

  const valid = isRangeValid();
  const nb = nbMonths();

  return (
    <Modal
      opened={opened}
      onClose={handleClose}
      title="Exporter le planning"
      size="sm"
    >
      <Stack gap="md">
        {/* ── Du mois ── */}
        <div>
          <Text size="sm" fw={500} mb={6}>
            Du mois
          </Text>
          <Group grow>
            <Select
              data={MONTH_OPTIONS}
              value={startMonth}
              onChange={(v) => v && setStartMonth(v)}
              placeholder="Mois"
            />
            <Select
              data={YEAR_OPTIONS}
              value={startYear}
              onChange={(v) => v && setStartYear(v)}
              placeholder="Année"
            />
          </Group>
        </div>

        {/* ── Au mois ── */}
        <div>
          <Text size="sm" fw={500} mb={6}>
            Au mois
          </Text>
          <Group grow>
            <Select
              data={MONTH_OPTIONS}
              value={endMonth}
              onChange={(v) => v && setEndMonth(v)}
              placeholder="Mois"
            />
            <Select
              data={YEAR_OPTIONS}
              value={endYear}
              onChange={(v) => v && setEndYear(v)}
              placeholder="Année"
            />
          </Group>
        </div>

        {/* ── Résumé ── */}
        {valid ? (
          <Text size="xs" c="dimmed" ta="center">
            {nb} mois — {fileLabel()}
          </Text>
        ) : (
          <Text size="xs" c="red" ta="center">
            La date de fin doit être après la date de début
          </Text>
        )}

        <Divider />

        {/* ── Erreur / succès ── */}
        {error && (
          <Alert
            icon={<IconAlertCircle size={14} />}
            color="red"
            variant="light"
          >
            {error}
          </Alert>
        )}
        {success && (
          <Alert color="green" variant="light">
            ✅ {success}
          </Alert>
        )}

        {/* ── Boutons d'export ── */}
        <Group grow>
          <Button
            leftSection={<IconFileSpreadsheet size={16} />}
            color="green"
            onClick={handleExportExcel}
            loading={loadingExcel}
            disabled={!valid || loadingPdf}
          >
            Excel
          </Button>
          <Button
            leftSection={<IconFileTypePdf size={16} />}
            color="red"
            onClick={handleExportPdf}
            loading={loadingPdf}
            disabled={!valid || loadingExcel}
          >
            PDF
          </Button>
        </Group>

        <Button variant="subtle" onClick={handleClose}>
          Fermer
        </Button>
      </Stack>
    </Modal>
  );
}
