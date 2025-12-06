# Control

**⚠️ In Development**  
This project is actively being worked on and is not yet considered stable or production‑ready.

Control is a Windows desktop application written in Rust that interfaces with a defense system over USB.  
The project was developed as part of a university group assignment and demonstrates user authentication,
real‑time sensor monitoring, and command transmission to the hardware.

## Features

- **USB communication** – Uses `serialport` for low‑level serial I/O.
- **GUI** – Built with `eframe/egui`, featuring custom widgets for boolean controls and indicators.
- **Authentication** – Simple username/password check with role based access (`View` / `Modify`).  
  *Bob* can modify, *god* has extended privileges.
- **Session timeout** – Sessions expire automatically after a configurable period.

## Building

```bash
# Clone the repo (already on your machine)
cd control

# Build in release mode
cargo build --release
```

The binary will be available under `target/release/control.exe`.

## Running

```bash
./target/release/control.exe
```

Once launched, log in using one of the following credentials:

| Username | Password |
|----------|----------|
| Bob      | 123      |
| god      | 123      |

After logging in you can toggle the alarm state and view battery status.

## Project Structure

```text
control/
├── assets/          # Font files for UI
├── embed/           # (unused, placeholder)
├── src/
│   ├── authorized/   # API wrappers
│   ├── fluent/       # Fluent UI helpers
│   ├── widgets/      # Custom egui widgets
│   ├── app.rs        # Main rendering logic
│   ├── auth.rs       # Authentication helper
│   ├── data.rs       # Shared state structs
│   └── main.rs       # Application entry point
├── Cargo.toml
└── README.md         # This file
```

## Dependencies

| Crate | Purpose |
|-------|---------|
| `eframe` & `egui` | GUI framework |
| `serialport` | USB serial communication |
| `chrono` | Time handling for sessions |
| `tokio` | Async runtime (future‑proofing) |

All dependencies are specified in `Cargo.toml`.

## Contributing

Feel free to open issues or pull requests.  
Keep changes small, well‑documented, and ensure the tests pass (if added).

--- 

© 2024 University Project – All rights reserved.