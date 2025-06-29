use std::fs;
use std::path::PathBuf;

pub fn get_config_dir() -> Option<PathBuf> {
    dirs::config_dir().map(|mut path| {
        path.push("taskclaw");
        path
    })
}

pub fn read_config_file(file_name: &str) -> Option<String> {
    if let Some(mut config_dir) = get_config_dir() {
        config_dir.push(file_name);
        fs::read_to_string(config_dir).ok()
    } else {
        None
    }
}

pub fn read_default_config() -> Option<String> {
    read_config_file("config.toml")
}
