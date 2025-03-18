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
use tauri::http::Response;
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
    .register_uri_scheme_protocol("local", move |uri_scheme_context, request| {
      let context = uri_scheme_context.app_handle().state::<Context>();
      let config = context.0.lock().unwrap();

      let uri = request.uri().to_string();
      let Ok(url) = LocalUrl::parse(&uri) else {
        return Response::builder().status(404).body("invalid url".as_bytes().to_vec()).unwrap();
      };

      let mut data: Vec<u8> = vec!();
      if note::read_attachment(&config, &url.note, &url.filename, &mut data).is_err() {
        return Response::builder().status(404).body("Unknown attachment".as_bytes().to_vec()).unwrap();
      };

      Response::builder()
        .status(200)
        .header("Content-Type", &url.mime_type)
        .body(data)
        .unwrap()
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
  config.write(name, value);
  Ok(())
}
