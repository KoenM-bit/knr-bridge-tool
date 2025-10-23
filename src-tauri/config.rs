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
        // Try loading .env from multiple locations (same logic as Bridge)
        let env_paths = vec![
            std::path::PathBuf::from("./.env"),                    // Current directory
            std::path::PathBuf::from("./src-tauri/.env"),         // Development location
            std::path::PathBuf::from("../src-tauri/.env"),        // In case running from parent
        ];
        
        let mut env_loaded = false;
        for env_path in &env_paths {
            if dotenvy::from_path(env_path).is_ok() {
                env_loaded = true;
                break;
            }
        }
        
        if !env_loaded {
            // Fallback to standard dotenv
            let _ = dotenvy::dotenv();
        }
        
        Self {
            robot_id: std::env::var("ROBOT_ID").unwrap_or_else(|_| "robot-1".into()),
            robot_base: std::env::var("ROBOT_BASE").unwrap_or_else(|_| "http://192.168.0.57:31950".into()),
        }
    }
}

impl Config {
    pub fn load() -> Self {
        // Always try to load environment variables first
        let default_cfg = Self::default();
        
        let path = Self::path();
        if let Ok(contents) = fs::read_to_string(&path) {
            if let Ok(mut saved_cfg) = serde_json::from_str::<Config>(&contents) {
                // Override saved config with environment variables if they exist
                if std::env::var("ROBOT_ID").is_ok() {
                    saved_cfg.robot_id = default_cfg.robot_id;
                }
                if std::env::var("ROBOT_BASE").is_ok() {
                    saved_cfg.robot_base = default_cfg.robot_base;
                }
                saved_cfg
            } else {
                default_cfg
            }
        } else {
            let cfg = default_cfg;
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