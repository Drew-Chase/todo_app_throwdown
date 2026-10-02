use tracing::{debug, error, info, trace, warn};

/// Bridge for webview-side logging.
///
/// The frontend logger (see `src/util/logger.ts`) invokes this command so that
/// `console.*` / `log.*` calls in React are re-emitted as `tracing` events
/// under the `frontend` target and land in the same rolling log files as the
/// Rust-side logs.
#[tauri::command]
pub async fn log(level: String, message: String, location: Option<String>) {
    match level.as_str() {
        "trace" => trace!(target: "frontend", location = ?location, "{message}"),
        "debug" => debug!(target: "frontend", location = ?location, "{message}"),
        "info" => info!(target: "frontend", location = ?location, "{message}"),
        "warn" => warn!(target: "frontend", location = ?location, "{message}"),
        "error" => error!(target: "frontend", location = ?location, "{message}"),
        _ => info!(target: "frontend", location = ?location, "{message}"),
    }
}