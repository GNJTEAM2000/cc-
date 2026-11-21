# NIGHTFALL 0.2.1

Standalone Rust desktop dashboard and offline sandbox. It does not attach to, inspect, or modify FiveM/VMP or any other game process.

## Features
- Dark purple egui interface
- Insert toggles the panel while the app window has keyboard focus
- Local CPU and RAM diagnostics
- Synthetic visual sandbox entities
- Offline aim/trigger/vehicle demonstration switches (UI-only simulations)
- JSON configuration saved under `%APPDATA%/nightfall/config.json` on Windows or `$HOME/nightfall/config.json` elsewhere

## Build
Requires a Rust toolchain and a supported desktop environment.

```sh
cargo check
cargo test
cargo run --release
```

## Test status
The source was statically reviewed. The sysinfo 0.32 CPU API calls were corrected (`refresh_cpu_usage` / `global_cpu_usage`), and memory values are now displayed in MiB. A real Cargo build and GUI launch have **not** been verified in the current environment because `cargo` and `rustc` are not installed here.

## GitHub CI
A GitHub Actions workflow runs `cargo check --all-targets` and `cargo test --all-targets` on pushes and pull requests.
