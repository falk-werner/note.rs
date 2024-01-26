// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs::DirEntry, io::{Error, Result}, path::{Path, PathBuf}};
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
  match get_readme_dirs(base_path) {
    Ok(readme_dirs) => {
      return readme_dirs;
    }
    Err(e) => { error_handling(e.to_string()); }
  }
  return Vec::<String>::new();
}

fn get_readme_dirs(base_path: PathBuf) -> Result<Vec<String>> {
  let mut readme_dirs = Vec::<String>::new();
  let dir_entries = std::fs::read_dir(base_path)?;
  for dir_entry in dir_entries {
    match get_readme_dir(dir_entry) {
      Ok(readme_dir) => readme_dirs.push(readme_dir),
      Err(e) => error_handling(e.to_string())
    }
  }
  return Ok(readme_dirs)
}

fn get_readme_dir(dir_entry: Result<DirEntry>) -> Result<String> {
  let dir_entry = dir_entry?;
  if dir_entry.file_type()?.is_dir() {
    if Path::new(&dir_entry.path()).join("README.md").is_file() {
      return Ok(dir_entry.file_name().into_string().unwrap());
    }
  }
  return Err(Error::new(std::io::ErrorKind::NotFound, "README.md not found."))
}

fn error_handling(e: String) {
  // implementation up to discussion
  eprintln!("{e}");
}
