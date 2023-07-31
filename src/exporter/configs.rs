use std::{error::Error, fs, sync::Mutex};

use log::debug;
use once_cell::sync::Lazy;
use serde::Deserialize;
use toml;

#[derive(Deserialize)]
pub struct Config {
    pub paths: GamePaths,
    pub api: ApiConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            paths: GamePaths::default(),
            api: ApiConfig::default(),
        }
    }
}

#[derive(Deserialize)]
pub struct GamePaths {
    pub save_location: String,
    pub game_files_dir: String,
    pub localisation_path: String,
}

impl Default for GamePaths {
    fn default() -> Self {
        Self {
            save_location: String::from(""),
            game_files_dir: String::from(""),
            localisation_path: String::from(""),
        }
    }
}

#[derive(Deserialize)]
pub struct ApiConfig {
    pub ip: String,
    pub port: u16,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            ip: String::from("0.0.0.0"),
            port: 8881,
        }
    }
}
// -------
pub static CONFIGS: Lazy<Mutex<Config>> = Lazy::new(|| Mutex::new(Config::default()));

// -------

pub fn read_configs() -> Result<Config, Box<dyn Error>> {
    debug!("Reading config file");
    let config_content = fs::read("config.toml")?;
    let config: Config = toml::from_str(String::from_utf8(config_content)?.as_str())?;
    Ok(config)
}
