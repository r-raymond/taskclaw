use log::{debug, info};
use std::fs;
use std::path::PathBuf;
use taskclaw::config::Config;

pub fn get_config_dir() -> Option<PathBuf> {
    let config_dir = dirs::config_dir().map(|mut path| {
        path.push("taskclaw");
        path
    });
    debug!("Config directory: {:?}", config_dir);
    config_dir
}

pub fn read_config() -> Config {
    info!("Reading config file");
    if let Some(mut config_dir) = get_config_dir() {
        config_dir.push("config.toml");
        if let Ok(content) = fs::read_to_string(config_dir) {
            if let Ok(config) = toml::from_str(&content) {
                info!("Config file loaded successfully");
                return config;
            }
        }
    }
    info!("No config file found, using default config");
    Config::default()
}