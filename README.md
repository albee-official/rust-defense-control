# Control

Control is a Windows desktop application written in Rust that communicates with a defense system over USB.
The project was completed as part of a university group assignment and is now read‑only; no further development will be
performed.

## Features

- **USB communication** – Uses the `serialport` crate for low‑level serial I/O. Allows changing connected device at
  runtime.
- **GUI** – Built with `eframe/egui`, featuring custom widgets for boolean controls and status indicators.
- **Authentication** – Simple username/password check with role‑based access (`View` / `Modify`).
- **Session timeout** – Sessions expire automatically after a configurable period.

## Project structure

```
/
├── assets/           # Font files for UI
├── src/
│   ├── authorized/   # API wrappers
│   ├── fluent/       # Fluent UI helpers
│   ├── widgets/      # Custom egui widgets
│   ├── app.rs        # Main rendering logic
│   ├── auth.rs       # Authentication helper
│   ├── data.rs       # Shared state structs
│   └── main.rs       # Application entry point
├── Cargo.toml
└── README.md
```

## Building

From the project root run:

```bash
cargo build --release
```

The executable will be located at `target/release/control.exe`.

## Running

After building, start the application with:

```bash
./target/release/control.exe
```

Once launched, log in using one of the following credentials:

| Username | Password                      |
|----------|-------------------------------|
| Sigma    | zZz_amogus_sussy_baka1337_zZz |

You can toggle the alarm state and view battery status.

## Dependencies

The project relies on the following crates:

- `eframe` & `egui` – GUI framework
- `serialport-rs` – USB serial communication
- `chrono` – Time handling for sessions
- `anyhow` – Unified result type

All dependencies are listed in `Cargo.toml`.

## Status

This repository is now read‑only. The project has been finished and will not receive further updates.

---
