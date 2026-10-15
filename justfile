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
    @New-Item -Path .\target\dist -Type Directory -Force
    @Copy-Item .\target\release\*.exe .\target\dist\
[linux]
dist:build
    mkdir -p ./target/dist
    find target/release -maxdepth 1 -type f -executable -exec cp -t target/dist {} +