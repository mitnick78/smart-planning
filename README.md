# 📅 Smart Planning Pro

> Application de bureau pour gérer, visualiser et exporter vos plannings personnels ou familiaux.

![Version](https://img.shields.io/badge/version-0.1.2-blue)
![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows-lightgrey)
![License](https://img.shields.io/badge/license-MIT-green)

---

## ✨ Fonctionnalités

### 🗓️ Calendrier interactif

- Navigation mois par mois
- Affichage des créneaux par période (**Matin / Après-midi / Soir / Journée**)
- Clic sur un jour pour assigner ou supprimer un label
- Indicateur visuel du jour en cours

### 🏷️ Labels personnalisés

- Création de labels avec un nom et une couleur personnalisée
- Suppression avec option de vider les jours associés
- Persistance complète en base de données locale (SQLite)

### 🏖️ Vacances scolaires (France)

- Sélecteur de zone (**A / B / C**)
- Synchronisation automatique depuis le calendrier officiel du [Ministère de l'Éducation Nationale](https://www.education.gouv.fr/calendrier-scolaire-toutes-les-dates-des-cours-et-des-vacances-100148)
- Affichage en jaune pastel sur le calendrier
- Cumul possible : si un jour est déjà labelisé, la couleur du label est prioritaire et un badge 🏖️ indique la période de vacances

### 📊 Statistiques

- Décompte automatique des jours par label sur le mois affiché
- Mise à jour en temps réel

### 📤 Export

| Format            | Contenu                                                                                |
| ----------------- | -------------------------------------------------------------------------------------- |
| **Excel (.xlsx)** | Date · Label (« Vacances scolaires » pour les jours sans label en période de vacances) |
| **PDF**           | Calendrier visuel multi-mois en A4 paysage, avec couleurs des labels                   |

Les fichiers sont enregistrés dans **Documents / Smart Planning Pro** sur votre ordinateur.

---

## 💻 Installation

### Télécharger l'application

Rendez-vous sur la page [**Releases**](https://github.com/mitnick78/smart-planning-pro/releases/latest) et téléchargez le fichier correspondant à votre système :

| Système                             | Fichier à télécharger                    |
| ----------------------------------- | ---------------------------------------- |
| macOS (Apple Silicon — M1/M2/M3/M4) | `Smart.Planning.Pro_0.1.2_aarch64.dmg`   |
| macOS (Intel)                       | `Smart.Planning.Pro_0.1.2_x86_64.dmg`    |
| Windows                             | `Smart.Planning.Pro_0.1.2_x64-setup.exe` |

### macOS — première ouverture

Apple peut bloquer l'application car elle n'est pas signée. Pour l'ouvrir :

1. **Double-cliquez** sur le `.dmg` et faites glisser l'app dans **Applications**
2. Faites **clic droit → Ouvrir** sur l'icône dans Applications
3. Cliquez **Ouvrir** dans la boîte de dialogue qui apparaît

> Cette étape n'est nécessaire qu'une seule fois.

### Windows

Double-cliquez sur le fichier `.exe` et suivez l'assistant d'installation.

---

## 🚀 Utilisation rapide

### 1. Créer un label

Dans le panneau gauche, cliquez sur **"+"**, saisissez un nom et choisissez une couleur. Le label est sauvegardé immédiatement.

### 2. Assigner un label à un jour

- Sélectionnez un label dans le panneau gauche (il devient actif)
- Cliquez sur un jour du calendrier
- Choisissez le créneau (Matin, Après-midi, Soir, ou Journée)

### 3. Activer les vacances scolaires

- Sélectionnez votre zone (A, B ou C) dans la barre du calendrier
- Cliquez sur 🔄 pour synchroniser — une connexion internet est requise
- Les jours de vacances apparaissent en jaune pastel

### 4. Exporter

- Cliquez sur **Export Excel** ou **Export PDF** dans la barre d'outils
- Choisissez la plage de mois à exporter
- Le fichier est généré dans **Documents / Smart Planning Pro**

---

## 🛠️ Stack technique

| Composant             | Technologie                                                |
| --------------------- | ---------------------------------------------------------- |
| Framework desktop     | [Tauri 2](https://tauri.app/) (Rust)                       |
| Interface utilisateur | React 18 + TypeScript                                      |
| UI Components         | [Mantine](https://mantine.dev/)                            |
| Base de données       | SQLite (via `rusqlite`)                                    |
| Export Excel          | `rust_xlsxwriter`                                          |
| Export PDF            | `printpdf`                                                 |
| Vacances scolaires    | [API Éducation Nationale](https://data.education.gouv.fr/) |

---

## 🏗️ Développement local

### Prérequis

- [Node.js](https://nodejs.org/) 18+
- [Rust](https://www.rust-lang.org/tools/install) (stable)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites)

### Lancer en mode développement

```bash
# Cloner le dépôt
git clone https://github.com/mitnick78/smart-planning-pro.git
cd smart-planning-pro

# Installer les dépendances frontend
cd frontend
npm install

# Lancer l'application
npm run tauri dev
```

### Compiler pour la production

```bash
npm run tauri build
```

Les binaires sont générés dans `src-tauri/target/release/bundle/`.

---

## 📁 Structure du projet

```
smart-planning-pro/
├── frontend/                  # Interface React/TypeScript
│   └── src/
│       ├── components/        # Composants UI (Calendar, Labels, Stats…)
│       ├── hooks/             # Logique métier (usePlanning, useSchoolHolidays…)
│       └── store/             # État global (Zustand)
└── src-tauri/                 # Backend Rust
    └── src/
        ├── commands.rs        # Commandes Tauri (bridge frontend ↔ backend)
        ├── database/          # Initialisation SQLite
        ├── repositories/      # Accès aux données
        ├── export_service.rs  # Export Excel
        └── pdf_export_service.rs  # Export PDF
```

---

## 📋 Changelog

### v0.1.2

- ✅ Ajout des vacances scolaires (zones A/B/C, sync API officielle)
- ✅ Badge 🏖️ sur les jours avec label + vacances
- ✅ Correction de l'export PDF (fichier vide)
- ✅ Lien vers le calendrier officiel education.gouv.fr
- ✅ Build macOS Intel (x86_64) ajouté

### v0.1.1

- ✅ Export Excel et PDF
- ✅ Statistiques par mois
- ✅ Labels personnalisés avec couleurs

### v0.1.0

- 🎉 Première version publique
- Calendrier interactif avec labels
- Base de données SQLite locale

---

## 🤝 Contribuer

Les contributions sont les bienvenues ! N'hésitez pas à :

- Ouvrir une [issue](https://github.com/mitnick78/smart-planning-pro/issues) pour signaler un bug ou proposer une fonctionnalité
- Soumettre une Pull Request

---

## 📄 Licence

Ce projet est sous licence [MIT](LICENSE).

---

<p align="center">Fait avec ❤️ par <a href="https://christophe-milliere.fr">Christophe Millière</a></p>
