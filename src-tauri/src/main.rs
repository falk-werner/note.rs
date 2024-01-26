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
async fn list() -> Vec<String> {
  let base_path = Path::new(home_dir().unwrap().as_path()).join(".notes");
  let mut notes = Vec::<String>::new();  
  // read all files in dir
  match std::fs::read_dir(base_path) {
    Ok(dir_entries) => {
      for dir_entry in dir_entries {
        match dir_entry {
          Ok(dir_entry) => {
            match dir_entry.file_type() {
              Ok(file_type) => {
                if file_type.is_dir() {
                  let readme_path = Path::new(&dir_entry.file_name()).join("README.md");
                  if readme_path.as_path().exists() && readme_path.to_str().is_some() {
                    notes.push(readme_path.to_str().unwrap_or_default().to_string())
                  } else {
                    error_handling(format!("{readme_path:?}"));
                  }
                }
              }
              Err(e) => { error_handling(e.to_string()); }
            }
          }
          Err(e) => { error_handling(e.to_string()); }
        }
      }
    }
    Err(e) => { error_handling(e.to_string()); }
  }
  return notes
}

fn error_handling(e: String) {
  // implementation up to discussion
  eprintln!("{e}");
}
