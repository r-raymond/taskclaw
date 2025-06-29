use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Config {
    pub store_location: Option<PathBuf>,
}

impl Config {
    pub fn get_task_store_path(&self) -> Option<PathBuf> {
        self.store_location.as_ref().map(|p| p.join("tasks"))
    }
}