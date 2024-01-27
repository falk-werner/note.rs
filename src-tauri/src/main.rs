// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod note;

use config::{Config};
use note::{Notebook};

fn main() {
  let config = Config::from_default_file();
  let notebook = Notebook::new(config);

  tauri::Builder::default()
    .manage(notebook)
    .invoke_handler(tauri::generate_handler![
      list
      ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

/// list the names of the notes
/// 
/// list all directories in the base directory that have a `README.md` file inside
#[tauri::command]
async fn list(notebook: tauri::State<'_, Notebook>) -> std::result::Result<Vec<String>, String> {
  Ok(notebook.list_notes())
}

