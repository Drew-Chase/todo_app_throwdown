use tracing::{ info};
use util::logging::log;

mod util;

pub static DEBUG: bool = cfg!(debug_assertions);

// webkit2gtk defaults to a DMA-BUF renderer that only supports X11. Under a
// native Wayland session (as opposed to XWayland) this produces a blank or
// black window on many GPU drivers, most notably NVIDIA. Disabling it makes
// webkit fall back to a renderer that works on both X11 and Wayland. This
// must be set before the webview is created, so it runs first in `run()`.
#[cfg(target_os = "linux")]
fn fix_linux_wayland_rendering() {
    if std::env::var_os("WAYLAND_DISPLAY").is_some() {
        // SAFETY: called once, single-threaded, before any webview/window is
        // created, so there is no concurrent access to the environment.
        unsafe {
            std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[cfg(target_os = "linux")]
    fix_linux_wayland_rendering();

    if let Err(e) = util::logging::setup_logging() {
        eprintln!("failed to initialise logging: {e}");
    }
    if let Err(e) = color_eyre::install() {
        eprintln!("failed to install color-eyre: {e}");
    }

    info!(
        "Starting {} build of the application...",
        if DEBUG { "development" } else { "production" }
    );

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![ log])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
