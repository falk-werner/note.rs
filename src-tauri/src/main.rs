// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::Path;

use tauri::api::path::home_dir;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      list
      ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

/// list all possible README.md locations in dirs of the save dir
#[tauri::command]
async fn list() -> Result<Vec<String>, String> {
  let base_path = Path::new(home_dir().unwrap().as_path()).join(".notes");
  
  let mut notes = Vec::<String>::new();

  // read all files in dir
  match std::fs::read_dir(base_path) {
    Ok(dir_entries) => {
      for dir_entry in dir_entries {
        match dir_entry {
          Ok(dir_entry) => {
            // filter for directories
            match dir_entry.file_type() {
              Ok(file_type) => {
                if file_type.is_dir() {
                  // add README.md to the found dir names and save them in list
                  notes.push(Path::new(&dir_entry.file_name()).join("README.md").to_str().unwrap().to_string());
                }
              }
              Err(e) => { 
                // error while figuring out the type of each file in the dir
                return Err(e.to_string())
              }
            }
          }
          Err(e) => { 
            // error while reading each object in dir
            return Err(e.to_string()) 
          }
        }
      }
    }
    Err(e) => { 
      // error while reading the dir
      return Err(e.to_string())
    }
  }
  return Ok(notes)
}
