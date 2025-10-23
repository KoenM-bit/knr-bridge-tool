mod bridge;
mod config;
mod model;

use bridge::Bridge;
use config::Config;
use std::sync::{Arc, Mutex};
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};
use tokio::task;
use reqwest::multipart;

#[derive(Clone, Default)]
struct AppState {
    running: Arc<Mutex<bool>>,
}

#[tauri::command]
fn load_config() -> Config {
    Config::load()
}

#[tauri::command]
fn save_config(robot_id: String, robot_base: String) {
    let cfg = Config { robot_id, robot_base };
    cfg.save();
}

/// Start de achtergrondloop (bridge)
#[tauri::command]
async fn start_bridge(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let mut running = state.running.lock().unwrap();
    if *running {
        return Ok(()); // al bezig
    }
    *running = true;

    let cfg = Config::load();
    task::spawn(async move {
        let bridge = Bridge::new(cfg);
        if let Err(e) = bridge.run().await {
            eprintln!("Bridge crashed: {:?}", e);
        }
    });

    Ok(())
}

/// Stoppen is optioneel — bridge loopt async, hier enkel flag terugzetten
#[tauri::command]
fn stop_bridge(state: tauri::State<'_, AppState>) {
    let mut running = state.running.lock().unwrap();
    *running = false;
}

/// Pick a local .py file, upload to robot, create a run (idle), and return a small summary.
#[tauri::command]
async fn test_upload() -> Result<String, String> {
    // Pick a .py file using a native dialog
    let file = rfd::FileDialog::new()
        .add_filter("Python", &["py"]) 
        .pick_file()
        .ok_or_else(|| "No file selected".to_string())?;

    let cfg = Config::load();
    let robot_base = cfg.robot_base.trim_end_matches('/').to_string();
    let data = std::fs::read(&file).map_err(|e| e.to_string())?;

    let client = reqwest::Client::new();
    // Upload protocol
    let form = multipart::Form::new().part(
        "files",
        multipart::Part::bytes(data.clone())
            .file_name("protocol.py")
            .mime_str("text/x-python").map_err(|e| e.to_string())?
    );

    let res = client
        .post(format!("{}/protocols", robot_base))
        .header("opentrons-version", "4")
        .header("accept", "application/json")
        .multipart(form)
        .send().await.map_err(|e| e.to_string())?;
    if !res.status().is_success() {
        return Err(format!("Upload failed: {}", res.status()));
    }
    let v: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
    let protocol_id = v["data"]["id"].as_str().unwrap_or("").to_string();

    // Create run (idle)
    let res = client
        .post(format!("{}/runs", robot_base))
        .header("opentrons-version", "4")
        .header("accept", "application/json")
        .json(&serde_json::json!({"data": {"protocolId": protocol_id}}))
        .send().await.map_err(|e| e.to_string())?;
    if !res.status().is_success() {
        return Err(format!("Run create failed: {}", res.status()));
    }
    let v: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
    let run_id = v["data"]["id"].as_str().unwrap_or("").to_string();
    Ok(format!("Uploaded. Protocol: {} | Run: {}", protocol_id, run_id))
}

#[tokio::main]
async fn main() {
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("settings", "Settings"))
        .add_item(CustomMenuItem::new("restart", "Restart Bridge"))
        .add_item(CustomMenuItem::new("quit", "Quit"));
    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            load_config,
            save_config,
            start_bridge,
            stop_bridge,
            test_upload
        ])
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "settings" => {
                    let _ = tauri::WindowBuilder::new(
                        app,
                        "settings",
                        tauri::WindowUrl::App("settings.html".into()),
                    )
                    .title("Robot Settings")
                    .resizable(false)
                    .build();
                }
                "restart" => {
                    let app_handle = app.app_handle();
                    task::spawn(async move {
                        let state = app_handle.state::<AppState>();
                        stop_bridge(state.clone());
                        let _ = start_bridge(state).await;
                    });
                }
                "quit" => std::process::exit(0),
                _ => {}
            },
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("App failed");
}