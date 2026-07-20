.PHONY: dev build build-arm build-intel build-all clean install test help

# ── Développement ─────────────────────────────────────────────
dev:
	cargo tauri dev

# ── Installation des dépendances ──────────────────────────────
install:
	cd frontend && npm install
	rustup target add aarch64-apple-darwin
	rustup target add x86_64-apple-darwin

# ── Build ─────────────────────────────────────────────────────

## Build pour Apple Silicon (M1/M2/M3)
build-arm:
	MACOSX_DEPLOYMENT_TARGET=11.0 cargo tauri build --target aarch64-apple-darwin
	@echo "DMG Apple Silicon : src-tauri/target/aarch64-apple-darwin/release/bundle/dmg/"

## Build pour Intel
build-intel:
	MACOSX_DEPLOYMENT_TARGET=11.0 cargo tauri build --target x86_64-apple-darwin
	@echo "DMG Intel : src-tauri/target/x86_64-apple-darwin/release/bundle/dmg/"

## Build pour les deux architectures
build-all: build-arm build-intel
	@echo "Les deux builds sont terminés !"

## Build par défaut (architecture native)
build:
	cargo tauri build
	@echo "Build terminé !"

# ── Tests ─────────────────────────────────────────────────────
test:
	cd src-tauri && cargo test

# ── Fix icônes (requis avant de builder) ──────────────────────
fix-icons:
	pip3 install Pillow --break-system-packages
	python3 -c "\
from PIL import Image; \
import os; \
[Image.open(os.path.join('src-tauri/icons', f)).convert('RGBA').save(os.path.join('src-tauri/icons', f)) \
for f in os.listdir('src-tauri/icons') if f.endswith('.png')]; \
print('Icônes converties en RGBA')"

# ── Copie les DMG dans Downloads ──────────────────────────────
copy-arm:
	cp src-tauri/target/aarch64-apple-darwin/release/bundle/dmg/*.dmg ~/Downloads/
	@echo "DMG Apple Silicon copié dans ~/Downloads/"

copy-intel:
	cp src-tauri/target/x86_64-apple-darwin/release/bundle/dmg/*.dmg ~/Downloads/
	@echo "DMG Intel copié dans ~/Downloads/"

copy-all: copy-arm copy-intel
	@echo "Les deux DMG copiés dans ~/Downloads/"

# ── Nettoyage ─────────────────────────────────────────────────
clean:
	cd src-tauri && cargo clean
	rm -rf frontend/dist
	@echo "Nettoyage terminé !"

# ── Release complète ──────────────────────────────────────────
release: fix-icons build-all copy-all
	@echo "Release complète dans ~/Downloads/ !"

# ── Aide ──────────────────────────────────────────────────────
help:
	@echo ""
	@echo "Smart Planning Pro — Commandes disponibles"
	@echo "────────────────────────────────────────────"
	@echo "  make dev          → Lance l'app en développement"
	@echo "  make install      → Installe toutes les dépendances"
	@echo "  make build        → Build natif"
	@echo "  make build-arm    → Build Apple Silicon (M1/M2/M3)"
	@echo "  make build-intel  → Build Intel"
	@echo "  make build-all    → Build les deux architectures"
	@echo "  make test         → Lance les tests Rust"
	@echo "  make fix-icons    → Convertit les icônes en RGBA"
	@echo "  make copy-arm     → Copie DMG Apple Silicon dans Downloads"
	@echo "  make copy-intel   → Copie DMG Intel dans Downloads"
	@echo "  make copy-all     → Copie les deux DMG dans Downloads"
	@echo "  make release      → Build + copie tout dans Downloads"
	@echo "  make clean        → Nettoie les fichiers de build"
	@echo ""