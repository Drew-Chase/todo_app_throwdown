set windows-shell := ["powershell.exe", "-NoLogo", "-NoProfile", "-Command"]
set shell := ["bash", "-c"]

@default:
    just --list

[working-directory("crates/tauri")]
tauri:
    @just dev

iced:
    @cargo run --profile dev --package iced_todo_app

build: build-tauri
    @cargo build --release

build-tauri:
    @cargo tauri build --no-bundle

[windows]
dist: build
    @New-Item -Path ./target/dist -Type Directory -Force
    @Copy-Item ./target/release/*.exe ./target/dist/
[linux]
dist:build
    mkdir -p ./target/dist
    cp ./target/release/egui_todo_app ./target/release/gpui_todo_app ./target/release/iced_todo_app ./target/release/tauri_todo_app ./target/dist/