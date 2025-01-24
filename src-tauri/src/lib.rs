use std::fs::OpenOptions;
use std::io::{self, Seek, SeekFrom, Write};
use serde::Deserialize;
use std::fs;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Deserialize, Debug)]
struct Settings {
    path: String,
    devs: String,
    theme: String,
}

fn read_settings_from_file(file_path: &str) -> Result<Settings, String> {
    let file_content = fs::read_to_string(file_path).map_err(|e| format!("Error reading file: {}", e))?;
    let settings: Settings = serde_json::from_str(&file_content).map_err(|e| format!("Error parsing JSON: {}", e))?;
    Ok(settings)
}

#[tauri::command]
fn print_settings() -> Result<String, String> {
    let settings = read_settings_from_file("settings.json")?; //src\assets\settings.json
    println!("Settings: {:?}", settings);
    Ok(format!("Settings: {:?}", settings))
}

#[tauri::command]
fn handle_selection(selected: String) -> Result<String, String> {
    println!("Selected value: {}", selected);
    Ok(format!("Received: {}", selected))
}

#[tauri::command]
fn modify_value_at_offset(file_path: &str, offset: u64, new_value: u8) -> Result<String, String> {
    match OpenOptions::new().read(true).write(true).open(file_path) {
        Ok(mut file) => {
            if let Err(e) = file.seek(SeekFrom::Start(offset)) {
                return Err(format!("Seek error: {}", e));
            }
            if let Err(e) = file.write_all(&[new_value]) {
                return Err(format!("Write error: {}", e));
            }
            Ok(format!("Value at offset {:#X} modified to {:#X}", offset, new_value))
        }
        Err(e) => Err(format!("File open error: {}", e)),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, print_settings, handle_selection, modify_value_at_offset])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
