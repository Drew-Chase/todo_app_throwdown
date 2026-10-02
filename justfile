set windows-shell := ["powershell.exe", "-NoLogo", "-NoProfile","-Command"]
set shell := ["bash", "-c"]

[working-directory:"crates/tauri"]
tauri:
    @just dev
