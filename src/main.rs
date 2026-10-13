// Release builds run without a console window; Akoyo lives in the system tray.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;
mod auth;
mod config;
mod db;
mod dialog;
mod error;
mod instance;
mod logging;
mod server;
mod tray;
mod web;

use std::{io, net::TcpListener, time::Duration};

use anyhow::Context;
use tracing::{error, info, warn};

use crate::config::{Config, Paths};

fn main() {
    if let Err(err) = run() {
        error!("{err:#}");
        dialog::error(&format!("Akoyo could not start.\n\n{err:#}"));
        std::process::exit(1);
    }
}

fn run() -> anyhow::Result<()> {
    let paths = Paths::resolve()?;
    let _log_guard = logging::init(&paths.log_dir)?;
    info!(
        version = env!("CARGO_PKG_VERSION"),
        data_dir = %paths.data_dir.display(),
        "starting Akoyo"
    );

    let config = Config::load_or_create(&paths.config_file)?;
    let url = config.local_url();

    let listener = match TcpListener::bind(config.bind_addr()) {
        Ok(listener) => listener,
        Err(err) if err.kind() == io::ErrorKind::AddrInUse => {
            if instance::is_akoyo_running(config.port) {
                info!("Akoyo is already running, opening it in the browser");
                open_browser(&url);
                return Ok(());
            }
            anyhow::bail!(
                "Port {} is used by another program.\nChange \"port\" in {} and start Akoyo again.",
                config.port,
                paths.config_file.display()
            );
        }
        Err(err) => return Err(err).context("could not open the network port"),
    };
    listener.set_nonblocking(true)?;

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    let state = runtime.block_on(server::AppState::init(&paths))?;

    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel();
    let server = runtime.spawn(server::serve(listener, state, shutdown_rx));
    info!(%url, "Akoyo is running");

    if config.open_browser_on_start {
        open_browser(&url);
    }

    // Blocks until the user picks "Quit" in the tray menu.
    tray::run(&url)?;

    info!("shutting down");
    let _ = shutdown_tx.send(());
    match runtime.block_on(tokio::time::timeout(Duration::from_secs(5), server)) {
        Ok(Ok(Ok(()))) => {}
        Ok(Ok(Err(err))) => error!("server error: {err:#}"),
        Ok(Err(err)) => error!("server task failed: {err}"),
        Err(_) => warn!("server did not stop in time"),
    }
    runtime.shutdown_timeout(Duration::from_secs(1));
    Ok(())
}

pub(crate) fn open_browser(url: &str) {
    if let Err(err) = webbrowser::open(url) {
        warn!("could not open the browser: {err}");
    }
}
