use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Config {
    pub data_dir: Option<PathBuf>,
    pub data_format: Option<String>,
    pub default_priority: Option<String>,
    pub show_completed: Option<bool>,
}

impl Default for Config {
    fn default() -> Self {
        let default_data_dir = dirs::home_dir().map(|mut path| {
            path.push(".taskclaw");
            path
        });

        Self {
            data_dir: default_data_dir,
            data_format: Some("json".to_string()),
            default_priority: Some("medium".to_string()),
            show_completed: Some(true),
        }
    }
}

pub fn get_config_file() -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| {
        let mut home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        home.push(".config");
        home
    });
    path.push("claw");
    if std::fs::create_dir_all(&path).is_err() {
        eprintln!("Warning: Could not create config directory at {:?}", path);
    }
    path.push("config.toml");
    path
}

pub fn load_config() -> Config {
    let config_file = get_config_file();
    if config_file.exists() {
        let content = fs::read_to_string(&config_file).unwrap_or_default();
        toml::from_str(&content).unwrap_or_default()
    } else {
        let default_config = Config::default();
        save_config(&default_config);
        default_config
    }
}

pub fn save_config(config: &Config) {
    let config_file = get_config_file();
    let content = toml::to_string_pretty(config).unwrap();
    if let Err(e) = fs::write(&config_file, content) {
        eprintln!("Warning: Could not save config file: {}", e);
    }
}

pub fn load_config_from_path(path: &PathBuf) -> Result<Config, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}

pub fn save_config_to_path(
    config: &Config,
    path: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let content = toml::to_string_pretty(config)?;
    fs::write(path, content)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.data_format, Some("json".to_string()));
        assert_eq!(config.default_priority, Some("medium".to_string()));
        assert_eq!(config.show_completed, Some(true));
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let toml_str = toml::to_string(&config).unwrap();
        let deserialized: Config = toml::from_str(&toml_str).unwrap();
        assert_eq!(config, deserialized);
    }

    #[test]
    fn test_config_partial() {
        let toml_str = r#"
data_format = "yaml"
"#;
        let config: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(config.data_format, Some("yaml".to_string()));
        assert_eq!(config.default_priority, None);
        assert_eq!(config.show_completed, None);
    }

    #[test]
    fn test_save_and_load_config() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("test_config.toml");

        let original_config = Config {
            data_format: Some("yaml".to_string()),
            default_priority: Some("high".to_string()),
            show_completed: Some(false),
        };

        save_config_to_path(&original_config, &config_path).unwrap();
        let loaded_config = load_config_from_path(&config_path).unwrap();

        assert_eq!(original_config, loaded_config);
    }

    #[test]
    fn test_load_nonexistent_config() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("nonexistent.toml");

        let result = load_config_from_path(&config_path);
        assert!(result.is_err());
    }
}
