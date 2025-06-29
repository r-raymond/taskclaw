use std::fs;
use std::path::PathBuf;
use taskclaw::config::Config;

pub fn get_config_dir() -> Option<PathBuf> {
    dirs::config_dir().map(|mut path| {
        path.push("taskclaw");
        path
    })
}

pub fn read_config() -> Config {
    if let Some(mut config_dir) = get_config_dir() {
        config_dir.push("config.toml");
        if let Ok(content) = fs::read_to_string(config_dir) {
            if let Ok(config) = toml::from_str(&content) {
                return config;
            }
        }
    }
    Config::default()
}

