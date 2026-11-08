# Durability Tooltip — Horizon Native Mod

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Game: Minecraft 26.3](https://img.shields.io/badge/Minecraft-26.3-green.svg)](https://playhorizon.dev)
[![Engine: Horizon Nightly](https://img.shields.io/badge/Horizon-nightly--2ebf00a-orange.svg)](https://github.com/horizon-rust/horizon)
[![Format: .hnmod](https://img.shields.io/badge/Format-.hnmod%20v0.2.1-purple.svg)](https://playhorizon.dev/docs/mod-manifest)

A high-performance, native Rust mod for **Horizon Minecraft** (the open-source Minecraft Java Edition 26.3 engine rewrite).

**Durability Tooltip** displays dynamic remaining durability values, maximum durability, and health percentage directly inside item tooltips when hovering over any damageable item in your inventory, crafting tables, chests, and creative screens.

---

## ✨ Features

- **Accurate Numeric Durability:** Displays `remaining / max (percent%)`, e.g. `1561 / 1561 (100%)`.
- **Dynamic Color Indicators:**
  - 🟢 **Green** ($\ge 70\%$) — Pristine condition
  - 🟡 **Yellow** ($\ge 30\%$ and $< 70\%$) — Moderately damaged
  - 🔴 **Red** ($< 30\%$) — Critically damaged
- **Universal Internationalization (120+ Languages):**
  - Dynamically extracts localized labels via Minecraft's built-in `item.durability` translation key.
  - Full support for European languages (English `Durability`, Russian `Прочность`, German `Haltbarkeit`, Spanish `Durabilidad`, French `Durabilité`, etc.).
  - Full support for CJK full-width colons (`：` / `\u{ff1a}`) in Chinese Simplified/Traditional (`耐久度`) and Japanese (`耐久度`).
- **Comprehensive Item Support:**
  - Works on all tools, weapons, armor, shields, elytra, tridents, and new Minecraft 26.3 weapons (such as `minecraft:diamond_spear`).
  - Active for undamaged items (100%) as well as damaged items.
  - Automatically respects unbreakable items (`minecraft:unbreakable`).
- **Seamless Deduplication:**
  - Automatically replaces standard vanilla `F3+H` (Advanced Tooltips) durability lines without duplicating text.
- **In-Game Toggle (Options > Mods):**
  - Directly integrated into Horizon's native mod configuration screen with live `ON` / `OFF` toggling.
- **Zero Runtime Overhead:**
  - Pure client-side on-demand evaluation: 0 ticks, 0 background threads, 0 polling overhead.
  - Completely safe Rust: 0 `unsafe` blocks, 0 `unwrap()` panics on game state.

---

## 📁 Repository Structure

```text
durability-tooltip/
├── .gitignore                      # Git ignore rules (excludes target/ caches)
├── Cargo.lock                      # Dependency lockfile
├── Cargo.toml                      # Cargo manifest for standalone testing
├── compatibility.json              # Horizon engine & mixins compatibility specification
├── LICENSE                         # MIT License
├── package.py                      # Standalone deterministic packager (.hnmod)
├── README.md                       # Documentation & build guide
└── mixins/
    └── durability_tooltip/
        ├── horizon-mixin.toml      # Mixin bundle manifest (hook points & bindings)
        ├── runtime.rs              # Core durability math, formatting logic & unit tests
        ├── tooltip.rs              # Hook helper for Game::native_tooltip
        ├── list.rs                 # Mod list registration (Options > Mods)
        ├── settings.rs             # In-game settings button rendering
        └── click.rs                # In-game settings click toggle handler
```

---

## 🔧 Architecture & Injection Model

Horizon native mods are compiled directly into the game client via `horizon-mixins`:

| Hook Point | Kind | Target | Purpose |
| :--- | :---: | :--- | :--- |
| `tooltip` | `HEAD` | `horizon_client::game::native_hooks::Game<'_>::native_tooltip` | Appends durability line to the tooltip text vector |
| `list` | `HEAD` | `horizon_client::game::native_hooks::Game<'_>::native_mods` | Registers the mod in Horizon's mod list |
| `settings` | `HEAD` | `horizon_client::game::native_hooks::Game<'_>::native_settings` | Renders `Durability Tooltip: ON/OFF` button |
| `click` | `HEAD` | `horizon_client::game::native_hooks::Game<'_>::native_settings_click` | Handles click event to toggle state |

---

## 📦 Compatibility Matrix

| Component | Target Version / Commit |
| :--- | :--- |
| **Game Version** | Minecraft Java Edition `26.3` |
| **Horizon Engine** | `nightly-2026-11-07-2ebf00a` (`2ebf00a0281c592f79191e76fce0e7d522c6f425`) |
| **Horizon Mixins** | `0d6b103c70633d8fd9443d8d0acd3aadaacedb06` (`horizon-mixins` v0.6.0) |
| **Rust Toolchain** | Rust `1.90+` / `1.99.1-x86_64-pc-windows-gnu` |
| **Manifest Schema**| Horizon Native Mod Schema `1` |

---

## 🛠️ Building & Verification

### 1. Run Unit Tests
The core math and formatting logic can be tested standalone without building the full game:
```bash
cargo test
```
Expected output:
```text
test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### 2. Package Reproducible `.hnmod` Bundle
Run the standalone packaging script:
```bash
python package.py --out ./dist
```
This produces a byte-for-byte deterministic package:
- **File:** `dist/durability_tooltip-0.2.1.hnmod`
- **Size:** `6,951 bytes`
- **SHA-256:** `b5b3fda981b6128024f0395e67ff220b865ec37918d8f13123c014acd15b60ac`

---

## 🚀 Installation

1. Copy `durability_tooltip-0.2.1.hnmod` into your Horizon instance's native mods directory:
   ```text
   %APPDATA%\horizon\instances\<instance_name>\horizon-native-mods\
   ```
   *(Or globally in `%APPDATA%\horizon\horizon-native-mods\`)*
2. Open **Horizon Launcher**, select your instance, and verify `durability_tooltip` is visible in the **Mods** tab.
3. Click **Собрать** (Build) to stage and compile the native client with MinGW.
4. Launch the game and enjoy numeric durability tooltips!

---

## 📄 License

This project is licensed under the [MIT License](LICENSE).
Feel free to use, modify, and distribute.
