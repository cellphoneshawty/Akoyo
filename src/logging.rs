use std::path::Path;

use tracing_appender::{
    non_blocking::WorkerGuard,
    rolling::{RollingFileAppender, Rotation},
};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

/// Logs go to daily files in the data folder (kept for a week), and also to the
/// console in debug builds. `AKOYO_LOG` overrides the filter, e.g. `AKOYO_LOG=debug`.
pub fn init(log_dir: &Path) -> anyhow::Result<WorkerGuard> {
    let appender = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_prefix("akoyo")
        .filename_suffix("log")
        .max_log_files(7)
        .build(log_dir)?;
    let (writer, guard) = tracing_appender::non_blocking(appender);

    let filter = EnvFilter::try_from_env("AKOYO_LOG")
        .unwrap_or_else(|_| EnvFilter::new("info,sqlx=warn"));

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().with_writer(writer).with_ansi(false))
        .with(cfg!(debug_assertions).then(fmt::layer))
        .init();

    Ok(guard)
}
