export interface Label {
  id: number | null;
  name: string;
  colorHex: string;
  createdAt: string;
}

export interface LabelCreateinput {
  name: string;
  colorHex: string;
}

export interface LabelUpdateInput {
  id: number;
  name: string;
  colorHex: string;
}
