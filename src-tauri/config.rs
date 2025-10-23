use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use tauri::api::path::app_data_dir;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub robot_id: String,
    pub robot_base: String,
}

impl Default for Config {
    fn default() -> Self {
        // Load .env file first
        let _ = dotenvy::dotenv();
        
        Self {
            robot_id: std::env::var("ROBOT_ID").unwrap_or_else(|_| "robot-1".into()),
            robot_base: std::env::var("ROBOT_BASE").unwrap_or_else(|_| "http://localhost:31950".into()),
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let mut path = Self::path();
        if let Ok(contents) = fs::read_to_string(&path) {
            serde_json::from_str(&contents).unwrap_or_default()
        } else {
            let cfg = Self::default();
            cfg.save();
            cfg
        }
    }

    pub fn save(&self) {
        let path = Self::path();
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let _ = fs::write(path, serde_json::to_string_pretty(self).unwrap());
    }

    pub fn path() -> PathBuf {
        let dir = app_data_dir(&tauri::Config::default()).unwrap();
        dir.join("config.json")
    }
}