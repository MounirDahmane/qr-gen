# ⬛ QrGen

A fast, native QR-code generator written in **Rust** with **egui**. Generate, customize and export QR codes for any text or URL — live preview, persistent history, full color & shape control, and high-quality exports.

---

## Screenshots

> **Light mode — Error-correction selector**
> ![Light mode](docs/light_mode.png)
> **Dark mode — Rounded modules with custom colors**
> ![Dark mode rounded](docs/dark_rounded.png)
> **Dark mode — Custom color QR code**
> ![Custom colors](docs/custom_colors.png)

---

## Highlights

* Live preview that updates as you type
* Square or rounded module shapes
* Full foreground / background color control
* Error-correction levels: **L, M, Q, H** (with tooltips)
* Export PNG at selectable resolutions: `256`, `512`, `1024`, `2048` px
* Export SVG (vector, infinitely scalable)
* One-click copy to clipboard (image)
* Persistent history saved across sessions
* Light / Dark themes with instant toggle
* `Ctrl+S` saves/export shortcut

---

## Quick Start

### Prerequisites

* Rust (stable) — install via `rustup` if needed

### Build from source

```bash
git clone https://github.com/MounirDahmane/QrGen.git
cd QrGen
cargo build --release
# binary: target/release/qrgen
```

### Run in dev

```bash
cargo run --release
```

---

## Usage (short)

1. Paste text or a URL in **TEXT / URL**.
2. Adjust **Error Correction**, **Module Shape**, **Size**, and **Colors**.
3. Click **⚡ Generate & Save to History** to add the entry to history.
4. Click **💾 Save** (or `Ctrl+S`) to export.
5. Click **📋 Copy** to copy the QR image to clipboard.
6. Select any entry in **History** to restore it.

---

## Project layout

```bash
src/
├── main.rs                # application entry (eframe)
├── utility.rs             # shared helpers, env/config helpers, small utilities
└── utility/
    ├── mod.rs             # utility module re-exports
    ├── app.rs             # UI layout and eframe app logic
    ├── history.rs         # history persistence (serde + JSON)
    ├── qr.rs              # QR rendering and export logic
    └── types.rs           # shared enums (SaveFormat, ModuleShape, ...)
```

## Dependencies (high level)

* `eframe` / `egui` — UI framework
* `qrcode` — QR-generation algorithm
* `image` — PNG raster rendering & export
* `arboard` — clipboard image support
* `rfd` — native file dialogs
* `serde` / `serde_json` — history serialization
* `dirs` — OS data directory resolution

---

## Development & environment tips

* Keep `history.json` in the OS data directory (use `dirs` crate) for cross-platform persistence.
* Use `cargo run --release` for realistic performance testing (egui runtime differs in debug).
* Add `RUST_LOG` and a tiny logging helper in `src/utility.rs` to enable debug tracing during development.

---
