import { useState } from "react";
import {
  Stack,
  Text,
  Button,
  Group,
  ColorSwatch,
  ActionIcon,
  Modal,
  TextInput,
  ColorPicker,
  Tooltip,
  Radio,
  Box,
  Divider,
} from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { notifications } from "@mantine/notifications";
import { IconPlus, IconTrash, IconPencil } from "@tabler/icons-react";
import { useLabels } from "@/hooks";
import type { Label } from "@/types";

export default function LabelsPanel() {
  const {
    labels,
    activeLabel,
    setActiveLabelId,
    createLabel,
    updateLabel,
    deleteLabel,
    isLoading,
  } = useLabels();

  // states for modals
  const [createOpened, { open: openCreate, close: closeCreate }] =
    useDisclosure(false);
  const [deleteOpened, { open: openDelete, close: closeDelete }] =
    useDisclosure(false);
  const [editOpened, { open: openEdit, close: closeEdit }] =
    useDisclosure(false);

  const [newName, setNewName] = useState("");
  const [newColor, setNewColor] = useState("#4dabf7");
  const [deleteMode, setDeleteMode] = useState<"label" | "all">("label");
  const [selectedLabel, setSelectedLabel] = useState<Label | null>(null);

  // create label
  const handleCreate = async () => {
    if (!newName.trim()) return;
    try {
      await createLabel({ name: newName.trim(), colorHex: newColor });
      notifications.show({
        message: `Label "${newName}" créé`,
        color: "green",
      });
      setNewName("");
      setNewColor("#4dabf7");
      closeCreate();
    } catch {
      notifications.show({
        message: "Erreur lors de la création",
        color: "red",
      });
    }
  };

  // delete label
  const handleDeleteConfirm = async () => {
    if (!selectedLabel?.id) return;
    try {
      await deleteLabel(selectedLabel.id, deleteMode === "all");
      notifications.show({
        message: `Label "${selectedLabel.name}" supprimé`,
        color: "green",
      });
      closeDelete();
      setSelectedLabel(null);
    } catch {
      notifications.show({
        message: "Erreur lors de la suppression",
        color: "red",
      });
    }
  };

  // update label
  const handleEdit = async () => {
    if (!selectedLabel?.id || !newName.trim()) return;
    try {
      await updateLabel({
        id: selectedLabel.id,
        name: newName.trim(),
        colorHex: newColor,
      });
      notifications.show({ message: "Label mis à jour", color: "green" });
      closeEdit();
    } catch {
      notifications.show({
        message: "Erreur lors de la mise à jour",
        color: "red",
      });
    }
  };

  return (
    <>
      <Stack gap="xs" h="100%">
        <Group justify="space-between" mb={4}>
          <Text size="xs" fw={500} c="dimmed" tt="uppercase">
            Labels
          </Text>
          <Tooltip label="Nouveau label">
            <ActionIcon size="sm" variant="subtle" onClick={openCreate}>
              <IconPlus size={14} />
            </ActionIcon>
          </Tooltip>
        </Group>
        <Stack gap={4} style={{ flex: 1, overflowY: "auto" }}>
          {labels.map((label) => (
            <Group
              key={label.id}
              px="xs"
              py={6}
              style={{
                borderRadius: 8,
                cursor: "pointer",
                background:
                  activeLabel?.id === label.id
                    ? "var(--mantine-color-dark-5)"
                    : "transparent",
              }}
              justify="space-between"
              onClick={() => setActiveLabelId(label.id)}
            >
              <Group gap="xs">
                <ColorSwatch color={label.colorHex} size={12} />
                <Text size="sm">{label.name}</Text>
              </Group>
              <Group gap={4} onClick={(e) => e.stopPropagation()}>
                <ActionIcon
                  size="xs"
                  variant="subtle"
                  onClick={() => {
                    setSelectedLabel(label);
                    setNewName(label.name);
                    setNewColor(label.colorHex);
                    openEdit();
                  }}
                >
                  <IconPencil size={11} />
                </ActionIcon>
                <ActionIcon
                  size="xs"
                  variant="subtle"
                  color="red"
                  onClick={() => {
                    setSelectedLabel(label);
                    setDeleteMode("label");
                    openDelete();
                  }}
                >
                  <IconTrash size={11} />
                </ActionIcon>
              </Group>
            </Group>
          ))}
        </Stack>
        <Divider />
        <Box
          p="xs"
          style={{
            background: "var(--mantine-color-dark-6)",
            borderRadius: 8,
            border: "1px solid var(--mantine-color-dark-4)",
          }}
        >
          <Text size="xs" c="dimmed" mb={4}>
            Label actif
          </Text>
          {activeLabel ? (
            <Group gap="xs">
              <ColorSwatch color={activeLabel.colorHex} size={10} />
              <Text size="sm" fw={500}>
                {activeLabel.name}
              </Text>
            </Group>
          ) : (
            <Text size="xs" c="dimmed">
              Aucun sélectionné
            </Text>
          )}
        </Box>
      </Stack>

      <Modal
        opened={createOpened}
        onClose={closeCreate}
        title="Nouveau label"
        size="sm"
      >
        <Stack gap="md">
          <TextInput
            label="Nom"
            placeholder="Ex: École, Maman..."
            value={newName}
            onChange={(e) => setNewName(e.target.value)}
            onKeyDown={(e) => e.key === "Enter" && handleCreate()}
            autoFocus
          />
          <div>
            <Text size="sm" fw={500} mb="xs">
              Couleur
            </Text>
            <ColorPicker
              format="hex"
              value={newColor}
              onChange={setNewColor}
              swatches={[
                "#4dabf7",
                "#f03e3e",
                "#40c057",
                "#f59f00",
                "#cc5de8",
                "#20c997",
                "#ff6b6b",
                "#74c0fc",
              ]}
            />
          </div>
          <Group justify="flex-end">
            <Button variant="subtle" onClick={closeCreate}>
              Annuler
            </Button>
            <Button
              onClick={handleCreate}
              loading={isLoading}
              disabled={!newName.trim()}
            >
              Créer
            </Button>
          </Group>
        </Stack>
      </Modal>

      <Modal
        opened={editOpened}
        onClose={closeEdit}
        title="Modifier le label"
        size="sm"
      >
        <Stack gap="md">
          <TextInput
            label="Nom"
            value={newName}
            onChange={(e) => setNewName(e.target.value)}
            autoFocus
          />
          <div>
            <Text size="sm" fw={500} mb="xs">
              Couleur
            </Text>
            <ColorPicker
              format="hex"
              value={newColor}
              onChange={setNewColor}
              swatches={[
                "#4dabf7",
                "#f03e3e",
                "#40c057",
                "#f59f00",
                "#cc5de8",
                "#20c997",
                "#ff6b6b",
                "#74c0fc",
              ]}
            />
          </div>
          <Group justify="flex-end">
            <Button variant="subtle" onClick={closeEdit}>
              Annuler
            </Button>
            <Button onClick={handleEdit} loading={isLoading}>
              Enregistrer
            </Button>
          </Group>
        </Stack>
      </Modal>
      <Modal
        opened={deleteOpened}
        onClose={closeDelete}
        title={`Supprimer "${selectedLabel?.name}" ?`}
        size="sm"
      >
        <Stack gap="md">
          <Text size="sm" c="dimmed">
            Que voulez-vous faire avec les jours associés à ce label ?
          </Text>
          <Radio.Group
            value={deleteMode}
            onChange={(v) => setDeleteMode(v as "label" | "all")}
          >
            <Stack gap="xs">
              <Radio value="label" label="Supprimer uniquement le label" />
              <Radio
                value="all"
                label="Supprimer le label et tous les jours associés"
                color="red"
              />
            </Stack>
          </Radio.Group>
          <Group justify="flex-end">
            <Button variant="subtle" onClick={closeDelete}>
              Annuler
            </Button>
            <Button
              color="red"
              onClick={handleDeleteConfirm}
              loading={isLoading}
            >
              Supprimer
            </Button>
          </Group>
        </Stack>
      </Modal>
    </>
  );
}
