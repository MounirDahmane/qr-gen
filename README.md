# ⬛ QrGen

A fast, native **QR code generator** written in **Rust** with **egui**.  
Generate, customize, and export QR codes for any text or URL with **live preview**, **persistent history**, **full color & shape customization**, and **high-quality exports**.

---

## Screenshots

**Light mode — Error correction selector**

<img width="600" height="755" alt="Screenshot from 2026-03-10 15-10-50" src="https://github.com/user-attachments/assets/c2d5c05b-d5ce-4a82-a1a6-3bfe982bcd40" />

<br><br>

**Dark mode — Rounded modules with custom colors**

<img width="600" height="755" alt="Screenshot from 2026-03-10 15-11-53" src="https://github.com/user-attachments/assets/6b53d5b5-bf4b-4c34-b01a-dada19c5590c" />

<br><br>

**Dark mode — Custom color QR code**

<img width="600" height="755" alt="Screenshot from 2026-03-10 15-13-00" src="https://github.com/user-attachments/assets/7dded1fd-c356-4a0c-801d-231064318f7a" />

---

## Highlights

- **Live preview** that updates while typing
- **Square or rounded** QR modules
- Full **foreground and background color customization**
- Error-correction levels: **L, M, Q, H** (with tooltips)
- PNG export at selectable resolutions: `256`, `512`, `1024`, `2048` px
- **SVG export** (vector, infinitely scalable)
- **Clipboard copy** for quick sharing
- **Persistent history** saved across sessions
- **Light / Dark theme** toggle
- **Keyboard shortcut**: `Ctrl + S` for quick export

---

## Quick Start

### Prerequisites

- **Rust (stable)** — install via `rustup`

### Build from source

```bash
git clone https://github.com/MounirDahmane/qr-gen
cd QrGen
cargo build --release
```

Binary will be located at:

```
target/release/qrgen
```

### Run

```bash
cargo run --release
```

---

## Usage

1. Enter text or a URL in **TEXT / URL**.
2. Adjust **Error Correction**, **Module Shape**, **Size**, and **Colors**.
3. Click **⚡ Generate & Save to History**.
4. Click **💾 Save** (or press `Ctrl + S`) to export the QR code.
5. Click **📋 Copy** to copy the image to the clipboard.
6. Select any entry in **History** to restore previous configurations.

---

## Project Structure

```
src/
├── main.rs                # Application entry point (eframe)
├── utility.rs             # Shared helpers and environment utilities
└── utility/
    ├── mod.rs             # Utility module re-exports
    ├── app.rs             # UI layout and main egui application logic
    ├── history.rs         # History persistence (serde + JSON)
    ├── qr.rs              # QR rendering and export logic
    └── types.rs           # Shared enums (SaveFormat, ModuleShape, etc.)
```

---

## Dependencies (high level)

- **eframe / egui** — GUI framework
- **qrcode** — QR code generation algorithm
- **image** — PNG rendering and export
- **arboard** — clipboard image support
- **rfd** — native file dialogs
- **serde / serde_json** — history serialization
- **dirs** — OS-specific data directory resolution

---

## Development Notes

- Store `history.json` in the **OS data directory** (via `dirs`) for cross-platform persistence.
- Use `cargo run --release` for realistic UI performance testing.
- Add `RUST_LOG` and a small logging helper in `src/utility.rs` for debug tracing during development.