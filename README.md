# 📅 Smart Planning Pro

> Application de bureau moderne pour la gestion et l'analyse de plannings personnalisés.

![Smart Planning Pro](https://img.shields.io/badge/version-1.0.0-blue)
![Rust](https://img.shields.io/badge/Rust-1.77+-orange)
![React](https://img.shields.io/badge/React-18+-61DAFB)
![Tauri](https://img.shields.io/badge/Tauri-2.0-24C8D8)
![License](https://img.shields.io/badge/license-MIT-green)

---

## ✨ Fonctionnalités

- 📆 **Calendrier dynamique** — Navigation fluide entre les mois avec affichage coloré des labels
- 🏷️ **Labels personnalisés** — Créez, modifiez et supprimez vos catégories avec couleurs personnalisées
- ⏰ **Gestion des périodes** — Assignez des labels par créneau : Matin, Après-midi, Soir ou Journée
- 📊 **Statistiques en temps réel** — Décompte automatique des créneaux par label et par période
- 📤 **Export Excel** — Exportez vos plannings en fichier `.xlsx` avec mise en forme colorée
- 💾 **Persistance locale** — Données stockées en SQLite, fonctionne entièrement hors ligne
- 🌙 **Thème sombre** — Interface moderne et élégante

---

## 📸 Aperçu

```
┌─────────────────────────────────────────────────────────────┐
│  Smart Planning Pro                                         │
├──────────────┬──────────────────────────────┬───────────────┤
│   LABELS     │         Avril 2026           │ STATISTIQUES  │
│              │  Lun  Mar  Mer  Jeu  Ven...  │               │
│ • École      │                              │ Total : 12    │
│ • Maman      │  [🌅École] [ ] [🌙Maman]    │               │
│ • Papy Michel│  [ ] [☀️École] [ ] [ ]      │ École  : 8j   │
│              │                              │ Maman  : 4j   │
│ Label actif  │                              │               │
│ [🔵 École]   │                              │ Exporter Excel│
└──────────────┴──────────────────────────────┴───────────────┘
```

---

## 🚀 Installation

### Téléchargement direct

Télécharge la dernière version depuis les [Releases GitHub](../../releases) :

| Plateforme                     | Fichier                                   |
| ------------------------------ | ----------------------------------------- |
| macOS Apple Silicon (M1/M2/M3) | `Smart.Planning.Pro_x.x.x_aarch64.dmg`    |
| macOS Intel                    | `Smart.Planning.Pro_x.x.x_x64.dmg`        |
| Windows                        | `Smart.Planning.Pro_x.x.x_x64_en-US.msi`  |
| Linux                          | `smart-planning-pro_x.x.x_amd64.AppImage` |

### macOS — Si l'app est bloquée

```
Clic droit sur l'app → Ouvrir → Ouvrir quand même
```

Ou via Terminal :

```bash
xattr -cr "/Applications/Smart Planning Pro.app"
```

---

## 🛠️ Développement

### Prérequis

- [Rust](https://rustup.rs/) 1.77+
- [Node.js](https://nodejs.org/) 20+
- [Tauri CLI](https://tauri.app/) 2.0

### Installation

```bash
# Clone le repo
git clone https://github.com/TON-USERNAME/smart-planning-pro.git
cd smart-planning-pro

# Installe les dépendances frontend
cd frontend && npm install && cd ..

# Lance en mode développement
cargo tauri dev
```

### Build

```bash
# macOS Apple Silicon
cargo tauri build --target aarch64-apple-darwin

# macOS Intel
cargo tauri build --target x86_64-apple-darwin

# Windows (depuis Windows)
cargo tauri build

# Linux (depuis Linux)
cargo tauri build
```

---

## 🏗️ Architecture

```
smart-planning-pro/
├── src-tauri/                  # Backend Rust
│   └── src/
│       ├── lib.rs              # Point d'entrée Tauri
│       ├── commands.rs         # Commandes Tauri (pont Rust ↔ React)
│       ├── export_service.rs   # Export Excel
│       ├── database/
│       │   └── mod.rs          # Connexion SQLite
│       ├── models/
│       │   └── mod.rs          # Structs Label, PlanningEntry, StatEntry
│       └── repositories/
│           ├── label_repository.rs
│           └── planning_repository.rs
│
└── frontend/                   # Frontend React
    └── src/
        ├── types/              # Interfaces TypeScript
        ├── api/                # Couche invoke() Tauri
        ├── store/              # État global Zustand
        ├── hooks/              # Logique métier
        └── components/         # Composants Mantine UI
            ├── calendar/
            ├── labels/
            └── stats/
```

---

## 🗄️ Base de données

SQLite stockée dans :

- **macOS** : `~/Library/Application Support/smart-planning-pro/smart_planning.db`
- **Windows** : `%APPDATA%\smart-planning-pro\smart_planning.db`
- **Linux** : `~/.local/share/smart-planning-pro/smart_planning.db`

### Schéma

```sql
CREATE TABLE labels (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    name       TEXT    NOT NULL,
    color_hex  TEXT    NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE planning (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    date       DATE    NOT NULL,
    period     TEXT    NOT NULL CHECK(period IN ('matin', 'après-midi', 'soir', 'journée')),
    label_id   INTEGER NOT NULL REFERENCES labels(id),
    note       TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

---

## 🧰 Stack Technique

### Backend

| Technologie                                        | Rôle                    |
| -------------------------------------------------- | ----------------------- |
| [Rust](https://www.rust-lang.org/)                 | Langage principal       |
| [Tauri 2](https://tauri.app/)                      | Framework desktop       |
| [SQLite](https://www.sqlite.org/)                  | Base de données locale  |
| [rusqlite](https://github.com/rusqlite/rusqlite)   | Driver SQLite pour Rust |
| [rust_xlsxwriter](https://docs.rs/rust_xlsxwriter) | Export Excel            |
| [serde](https://serde.rs/)                         | Sérialisation JSON      |

### Frontend

| Technologie                                   | Rôle            |
| --------------------------------------------- | --------------- |
| [React 18](https://react.dev/)                | Framework UI    |
| [TypeScript](https://www.typescriptlang.org/) | Typage statique |
| [Mantine](https://mantine.dev/)               | Design System   |
| [Zustand](https://zustand-demo.pmnd.rs/)      | État global     |
| [Vite](https://vitejs.dev/)                   | Bundler         |

---

## 🗺️ Roadmap

- [ ] Notes par créneau
- [ ] Vue hebdomadaire
- [ ] Undo / Redo
- [x] Export PDF
- [ ] Graphiques statistiques
- [ ] Gestion multi-planning
- [ ] Jours fériés automatiques

---

## 📄 Licence

MIT — voir [LICENSE](LICENSE)

---

## 👨‍💻 Auteur

Développé par **Christophe** — Mantes-la-Jolie 🇫🇷
