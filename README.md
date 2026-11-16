# Todo App Throwdown

The same todo app, built in as many languages and frameworks as possible, so the results can be compared side by side.

## Requirement

Every implementation must compile on Windows, macOS, and Linux.

## Folder convention

Each implementation lives in `apps/<language>/<framework>/`, for example `apps/cpp/qt`. Rust apps sit under `apps/rust/` and are members of the root Cargo workspace.

A folder that has not been started holds an empty `donot.ignore` file, because git does not track empty directories. Delete that file once you add real code to the folder.

## Status

- **Built**: working implementation in the repo
- **Started**: implementation in progress
- **Placeholder**: empty folder with `donot.ignore`, not started
- **Ruled out**: fails the requirement; listed for reference only

## Rust

| Framework | Folder | Status | Notes |
|---|---|---|---|
| Tauri | `apps/rust/tauri` (Rust backend in `apps/rust/tauri/src-tauri`) | Built | Web frontend in the OS webview |
| iced | `apps/rust/iced` | Built | Native Rust GUI |
| GPUI | `apps/rust/gpui` | Started | GPU-accelerated UI framework from Zed |
| egui | `apps/rust/egui` | Started | Immediate-mode GUI |

Shared Rust code lives in `crates/shared`.

## JavaScript

| Framework | Folder | Status | Notes |
|---|---|---|---|
| Electron | `apps/javascript/electron` | Placeholder | Chromium-based shell |

## C++

| Framework | Folder | Status | Notes |
|---|---|---|---|
| Qt | `apps/cpp/qt` | Placeholder | Qt Widgets or QML |
| Dear ImGui | `apps/cpp/imgui` | Placeholder | Immediate-mode GUI |
| wxWidgets | `apps/cpp/wxwidgets` | Placeholder | Native widgets |

## Python

| Framework | Folder | Status | Notes |
|---|---|---|---|
| Flet | `apps/python/flet` | Placeholder | Flutter-rendered UI; 1.0 released September 2026 |
| PySide6 | `apps/python/pyside6` | Placeholder | Official Qt for Python (LGPL) |
| Dear PyGui | `apps/python/dearpygui` | Placeholder | Dear ImGui wrapper |
| imgui-bundle | `apps/python/imgui-bundle` | Placeholder | Dear ImGui bindings; pick one of the two ImGui options |
| Tkinter | `apps/python/tkinter` | Placeholder | Built in; add CustomTkinter for a modern look |

## Go

| Framework | Folder | Status | Notes |
|---|---|---|---|
| Wails v3 | `apps/go/wails` | Placeholder | Go backend with a web frontend; closest to Tauri |
| Gio | `apps/go/gio` | Placeholder | Immediate-mode, GPU-accelerated |
| Fyne | `apps/go/fyne` | Placeholder | Widget toolkit; uses cgo, so cross-building is harder |

## Java

| Framework | Folder | Status | Notes |
|---|---|---|---|
| Swing + FlatLaf | `apps/java/swing` | Placeholder | The JFrame option; FlatLaf adds a modern look and dark mode |
| JavaFX | `apps/java/javafx` | Placeholder | FXML and CSS styling |
| SWT | `apps/java/swt` | Placeholder | Eclipse's native widgets via JNI |

## Kotlin

| Framework | Folder | Status | Notes |
|---|---|---|---|
| Compose Multiplatform | `apps/kotlin/compose-multiplatform` | Placeholder | Desktop is stable on all three OSes |

## C#

| Framework | Folder | Status | Notes |
|---|---|---|---|
| Avalonia | `apps/csharp/avalonia` | Placeholder | Recommended; XAML with Skia rendering |
| Uno Platform | `apps/csharp/uno` | Placeholder | WinUI-style XAML; check its Linux path first |
| Eto.Forms | `apps/csharp/eto-forms` | Placeholder | Native controls on each OS |
| .NET MAUI | `apps/csharp/maui` | Placeholder | No official Linux target; keep only if you accept a Linux exception |

## Other languages

| Framework | Folder | Status | Notes |
|---|---|---|---|
| Flutter (Dart) | `apps/dart/flutter` | Placeholder | Stable on all three desktop platforms |
| SwiftCrossUI (Swift) | `apps/swift/swiftcrossui` | Placeholder | Experimental; Windows support unclear |
| Lazarus (Object Pascal) | `apps/pascal/lazarus` | Placeholder | Native widgets; fast compiles, small binaries |
| Racket | `apps/racket/gui` | Placeholder | `racket/gui` native widgets; oddball |
| Odin + raylib | `apps/odin/raylib` | Placeholder | Immediate-mode; oddball |

## Ruled out

- **WPF, WinForms, WinUI 3**: Windows only
- **SwiftUI**: Apple platforms only
- **Delphi**: VCL is Windows only; FMX Linux depends on an add-on whose license is still being negotiated

## Suggested first batch

Wails, Gio, Flet, Compose Multiplatform, Avalonia, Flutter.

## Build checks

The root `justfile` has run recipes for `tauri`, `iced`, and `gpui` only. A real Windows, macOS, and Linux check needs a CI matrix with a runner for each OS, because macOS builds realistically can't be produced from Docker on other hosts.
