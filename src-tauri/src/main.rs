// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod note;
mod noteerror;

use config::{Config};
use note::{Note};
use noteerror::{NoteResult};

fn main() {
  let config = Config::from_default_file();
  let note = Note::new(config);

  tauri::Builder::default()
    .manage(note)
    .invoke_handler(tauri::generate_handler![
      list,
      read,
      create,
      rename,
      write,
      remove,
      read_tags,
      write_tags
      ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

/// list the names of the notes
/// 
/// list all directories in the base directory that have a `README.md` file inside
#[tauri::command]
async fn list(note: tauri::State<'_, Note>) -> NoteResult<Vec<String>> {
  note.list()
}

#[tauri::command]
async fn read(note: tauri::State<'_, Note>, name: &str) -> NoteResult<String> {
  note.read(name)
}

#[tauri::command]
async fn create(note: tauri::State<'_, Note>) -> NoteResult<String> {
  note.create()
}

#[tauri::command]
async fn rename(note: tauri::State<'_, Note>, old_name: &str, new_name: &str) -> NoteResult<()> {
  note.rename(old_name, new_name)
}

#[tauri::command]
async fn write(note: tauri::State<'_, Note>, name: &str, content: &str) -> NoteResult<()> {
  note.write(name, content)
}

#[tauri::command]
async fn remove(note: tauri::State<'_, Note>, name: &str) -> NoteResult<()> {
  note.remove(name)
}

#[tauri::command]
async fn read_tags(note: tauri::State<'_, Note>, name: &str) -> NoteResult<Vec<String>> {
  note.read_tags(name)
}

#[tauri::command]
async fn write_tags(note: tauri::State<'_, Note>, name: &str, tags: Vec<String>) -> NoteResult<()> {
  note.write_tags(name, &tags)
}
