use std::{
    fs,
    net::{Ipv4Addr, SocketAddr},
    path::{Path, PathBuf},
};

use anyhow::Context;
use serde::{Deserialize, Serialize};

/// Where Akoyo keeps its files. Defaults to `%LOCALAPPDATA%\Akoyo` on Windows;
/// `AKOYO_DATA_DIR` overrides it (handy for development and portable setups).
#[derive(Debug, Clone)]
pub struct Paths {
    pub data_dir: PathBuf,
    pub config_file: PathBuf,
    pub database_file: PathBuf,
    pub log_dir: PathBuf,
}

impl Paths {
    pub fn resolve() -> anyhow::Result<Self> {
        let data_dir = match std::env::var_os("AKOYO_DATA_DIR") {
            Some(dir) => PathBuf::from(dir),
            None => directories::ProjectDirs::from("", "", "Akoyo")
                .context("could not find a folder to store Akoyo data in")?
                .data_local_dir()
                .to_path_buf(),
        };
        let log_dir = data_dir.join("logs");
        fs::create_dir_all(&log_dir)
            .with_context(|| format!("could not create {}", log_dir.display()))?;

        Ok(Self {
            config_file: data_dir.join("config.toml"),
            database_file: data_dir.join("akoyo.db"),
            log_dir,
            data_dir,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Port the web interface is served on.
    pub port: u16,
    /// Let other devices on the home network (phones, laptops) open Akoyo.
    pub allow_network_access: bool,
    pub open_browser_on_start: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: 7700,
            allow_network_access: true,
            open_browser_on_start: true,
        }
    }
}

impl Config {
    pub fn load_or_create(path: &Path) -> anyhow::Result<Self> {
        if !path.exists() {
            let config = Self::default();
            fs::write(path, toml::to_string_pretty(&config)?)
                .with_context(|| format!("could not write {}", path.display()))?;
            return Ok(config);
        }
        let text = fs::read_to_string(path)
            .with_context(|| format!("could not read {}", path.display()))?;
        toml::from_str(&text).with_context(|| format!("{} contains a mistake", path.display()))
    }

    pub fn bind_addr(&self) -> SocketAddr {
        let ip = if self.allow_network_access {
            Ipv4Addr::UNSPECIFIED
        } else {
            Ipv4Addr::LOCALHOST
        };
        SocketAddr::from((ip, self.port))
    }

    pub fn local_url(&self) -> String {
        format!("http://localhost:{}", self.port)
    }
}
