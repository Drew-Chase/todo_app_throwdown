set windows-shell := ["powershell.exe", "-NoLogo", "-NoProfile","-Command"]
set shell := ["bash", "-c"]

[working-directory:"crates/tauri"]
tauri:
    @just dev

iced:
    @cargo run --profile dev --package iced_todo_app

build:build-tauri
    @cargo build --release


build-tauri:
    @cargo tauri build --no-bundle
