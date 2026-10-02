set windows-shell := ["powershell.exe", "-NoLogo", "-NoProfile","-Command"]
set shell := ["bash", "-c"]

[working-directory:"crates/tauri"]
tauri:
    @just dev

build:build-tauri
    @cargo build --release


build-tauri:
    @cargo tauri build --no-bundle