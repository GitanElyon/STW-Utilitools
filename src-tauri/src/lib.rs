use native_dialog::FileDialog;
use serde::Deserialize;
use serde_json::json;
use serde_json::Value;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom, Write};
use std::path::Path;

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
    let file_content =
        fs::read_to_string(file_path).map_err(|e| format!("Error reading file: {}", e))?;
    let settings: Settings =
        serde_json::from_str(&file_content).map_err(|e| format!("Error parsing JSON: {}", e))?;
    Ok(settings)
}

#[tauri::command]
fn print_settings() -> Result<String, String> {
    let settings = read_settings_from_file("src/settings.json")?;
    println!("Settings: {:?}", settings);
    Ok(format!("Settings: {:?}", settings))
}

#[tauri::command]
fn handle_selection(selected: String) -> Result<String, String> {
    // println!("Selected value: {}", selected);

    // Read the settings.json file
    let settings_path = Path::new("src/settings.json");
    let settings_content = fs::read_to_string(settings_path).map_err(|e| e.to_string())?;

    // Parse the JSON content
    let mut settings: Value = serde_json::from_str(&settings_content).map_err(|e| e.to_string())?;

    // save the previous selected value so we know what to change back to
    let previous_selected = settings["devs"].as_str().unwrap().to_string();

    // Update the "devs" item to the new selected value
    settings["devs"] = Value::String(selected.clone());

    // check if fortnite path is correct
    // default path is C:\Program Files\Epic Games\Fortnite\
    let current_path = settings["path"].as_str().unwrap().to_string();
    if !Path::new(&current_path).exists() {
        let alt_path = "C:\\Documents\\Fortnite\\";

        if !Path::new(alt_path).exists() {
            // if it cant find the default path, check the alt path
            // if it exists, update the path in settings.json
            println!("Could not find default path at: {}", current_path);
            settings["path"] = Value::String(alt_path.to_string());
        } else {
            // if it cant find the default path or the alt path, return an error
            println!("Could not find alt path at: {}", alt_path);
            return Err(format!("Could not find path at: {}", alt_path));
        }
    }

    // Write the updated content back to the file
    let updated_content = serde_json::to_string_pretty(&settings).map_err(|e| e.to_string())?;
    fs::write(settings_path, updated_content).map_err(|e| e.to_string())?;

    // Grab offsets from offets.json
    let offsets_path = Path::new("src/offsets.json");
    let offsets_content = fs::read_to_string(offsets_path).map_err(|e| e.to_string())?;

    // Parse the JSON content
    let offsets: Value = serde_json::from_str(&offsets_content).map_err(|e| e.to_string())?;
    // println!("{:?}", offsets);
    println!("{:?}", offsets["types"]["off"]["value"].as_str());

    println!("Previous Selected: {}", previous_selected);
    // Find the sepecific offset and offset value for the user selected value
    // checks if the user selected a value that is in offsets.json
    if let Some(selected_type) = offsets["types"]
        .as_array()
        .unwrap()
        .iter()
        .find(|&t| t["name"] == selected)
    {
        let on_value = selected_type["on"]["value"].as_str().unwrap();
        let offset_value = selected_type["offset"].as_str().unwrap();

        println!(
            "Selected '{}' 'on' value: {}, 'offset' value: {}",
            selected, on_value, offset_value
        );
        println!("Path: {}", settings["path"].as_str().unwrap());
        // modify_value_at_offset(&current_path, offset_value.parse().unwrap(), on_value.parse().unwrap());

    // if its not in offsets.json, the user has selected off, so we can set the offsets to their off values (previous_selected)
    } else if let Some(offsets) = offsets["types"]
        .as_array()
        .unwrap()
        .iter()
        .find(|&t| t["name"] == previous_selected)
    {
        // this means the user selected the "off" value and we can set the offets to their off value
        let off_value = offsets["off"]["value"].as_str().unwrap();
        let offset_value = offsets["offset"].as_str().unwrap();

        println!(
            "Selected '{}' 'off' value: {}, 'offset' value: {}",
            selected, off_value, offset_value
        );
        println!("Path: {}", settings["path"].as_str().unwrap());
        // modify_value_at_offset(&current_path, offset_value.parse().unwrap(), off_value.parse().unwrap());
    }

    Ok(format!("Received: {}", selected))
}

fn modify_value_at_offset(file_path: &str, offset: u64, new_value: u8) -> Result<String, String> {
    match OpenOptions::new().read(true).write(true).open(file_path) {
        Ok(mut file) => {
            if let Err(e) = file.seek(SeekFrom::Start(offset)) {
                return Err(format!("Seek error: {}", e));
            }
            if let Err(e) = file.write_all(&[new_value]) {
                return Err(format!("Write error: {}", e));
            }
            Ok(format!(
                "Value at offset {:#X} modified to {:#X}",
                offset, new_value
            ))
        }
        Err(e) => Err(format!("File open error: {}", e)),
    }
}

// #[tauri::command]
// fn modify_value_at_offset(file_path: &str, offset: u64, new_value: u8) -> Result<String, String> {
//     match OpenOptions::new().read(true).write(true).open(file_path) {
//         Ok(mut file) => {
//             if let Err(e) = file.seek(SeekFrom::Start(offset)) {
//                 return Err(format!("Seek error: {}", e));
//             }
//             if let Err(e) = file.write_all(&[new_value]) {
//                 return Err(format!("Write error: {}", e));
//             }
//             Ok(format!("Value at offset {:#X} modified to {:#X}", offset, new_value))
//         }
//         Err(e) => Err(format!("File open error: {}", e)),
//     }
// }

#[tauri::command]
fn open_files() {
    let directory = FileDialog::new()
        .set_title("Select a Directory")
        .show_open_single_dir();

    match directory {
        Ok(Some(path)) => {
            println!("You selected: {:?}", path);

            // Read the existing settings.json file
            let settings_path = "src/settings.json";
            let mut settings: serde_json::Value = serde_json::from_reader(
                File::open(settings_path).expect("Unable to open settings file"),
            )
            .expect("Unable to parse settings file");

            // Update the path in the settings
            settings["path"] = json!(path.to_str().unwrap());

            // Write the updated settings back to the file
            let mut file = File::create(settings_path).expect("Unable to create settings file");
            file.write_all(
                serde_json::to_string_pretty(&settings)
                    .expect("Unable to serialize settings")
                    .as_bytes(),
            )
            .expect("Unable to write to settings file");
        }
        Ok(None) => println!("No directory selected"),
        Err(e) => println!("Error: {:?}", e),
    }
}

#[tauri::command]
fn get_devs_value() -> Result<String, String> {
    let settings = read_settings_from_file("src/settings.json")?;
    Ok(settings.devs)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            print_settings,
            handle_selection,
            /*modify_value_at_offset,*/ open_files,
            get_devs_value
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
