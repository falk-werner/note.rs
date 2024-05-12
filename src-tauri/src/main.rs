// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod note;
mod noteerror;
mod localurl;

use config::{Config, ConfigValue};
use noteerror::{NoteResult};
use localurl::{LocalUrl};
use tauri::Manager;
use tauri::http::ResponseBuilder;
use std::sync::{Arc, Mutex};

struct Context (Arc<Mutex<Config>>);


fn main() {
  let context = Context(Arc::new(Mutex::new(Config::from_default_file())));
  tauri::Builder::default()
    .manage(context)
    .invoke_handler(tauri::generate_handler![
      list,
      read,
      create,
      rename,
      write,
      remove,
      read_tags,
      write_tags,
      open_note_directory,
      take_screenshot,
      read_all_settings,
      write_setting
      ])
    .register_uri_scheme_protocol("local", move |app, request| {
      let context = app.state::<Context>();
      let config = context.0.lock().unwrap();

      let Ok(url) = LocalUrl::parse(request.uri()) else {
        return Err("invalid url".into());
      };

      let mut data: Vec<u8> = vec!();
      if let Err(_) = note::read_attachment(&config, &url.note, &url.filename, &mut data) {
        return Err("Unknown attachment".into());
      };

      let response = ResponseBuilder::new()
      .status(200)
      .mimetype(&url.mime_type)
      .body(data)
      .unwrap();
      Ok(response)
      })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

/// list the names of the notes
/// 
/// list all directories in the base directory that have a `README.md` file inside
#[tauri::command]
async fn list(context: tauri::State<'_, Context>) -> NoteResult<Vec<String>> {
  let config = context.0.lock().unwrap();
  note::list(&config)
}

#[tauri::command]
async fn read(context: tauri::State<'_, Context>, name: &str) -> NoteResult<String> {
  let config = context.0.lock().unwrap();
  note::read(&config, name)
}

#[tauri::command]
async fn create(context: tauri::State<'_, Context>) -> NoteResult<String> {
  let config = context.0.lock().unwrap();
  note::create(&config)
}

#[tauri::command]
async fn rename(context: tauri::State<'_, Context>, old_name: &str, new_name: &str) -> NoteResult<()> {
  let config = context.0.lock().unwrap();
  note::rename(&config, old_name, new_name)
}

#[tauri::command]
async fn write(context: tauri::State<'_, Context>, name: &str, content: &str) -> NoteResult<()> {
  let config = context.0.lock().unwrap();
  note::write(&config, name, content)
}

#[tauri::command]
async fn remove(context: tauri::State<'_, Context>, name: &str) -> NoteResult<()> {
  let config = context.0.lock().unwrap();
  note::remove(&config, name)
}

#[tauri::command]
async fn read_tags(context: tauri::State<'_, Context>, name: &str) -> NoteResult<Vec<String>> {
  let config = context.0.lock().unwrap();
  note::read_tags(&config, name)
}

#[tauri::command]
async fn write_tags(context: tauri::State<'_, Context>, name: &str, tags: Vec<String>) -> NoteResult<()> {
  let config = context.0.lock().unwrap();
  note::write_tags(&config, name, &tags)
}

#[tauri::command]
async fn open_note_directory(context: tauri::State<'_, Context>, name: &str) -> NoteResult<()> {
  let config = context.0.lock().unwrap();
  note::open_note_directory(&config, name)
}

#[tauri::command]
async fn take_screenshot(context: tauri::State<'_, Context>, name: &str) -> NoteResult<String> {
  let config = context.0.lock().unwrap();
  note::take_screenshot(&config, name)
}

#[tauri::command]
async fn read_all_settings(context: tauri::State<'_, Context>) -> NoteResult<Vec<ConfigValue>> {
  let config = context.0.lock().unwrap();
  Ok(config.read_all())
}

#[tauri::command]
async fn write_setting(context: tauri::State<'_, Context>, name: &str, value: &str) -> NoteResult<()> {
  let mut config = context.0.lock().unwrap();
  Ok(config.write(name, value))
}
