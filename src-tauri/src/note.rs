use std::{fs, fs::DirEntry, fs::File, path::{Path, PathBuf}, io::Read, process::Command};
use crate::config::{Config};
use crate::noteerror::{NoteError, NoteResult};
use uuid::Uuid;

use opener;

/// Lists the names of the notes.
/// 
/// list all directories in the base directory that have a `README.md` file inside
pub fn list(config: &Config) -> Result<Vec<String>, NoteError> {
    let base_path = config.get_base_path();
    match get_note_names(base_path) {
      Ok(note_names) => return Ok(note_names),
      Err(e) => error_handling(e.to_string())
    }
    Ok(Vec::<String>::new())
}

/// Reads the contents of the given note.
pub fn read(config: &Config, name : &str) -> NoteResult<String> {
  let readme = Path::new(&get_note_path(&config, name)?).join("README.md");
  Ok(fs::read_to_string(readme)?)
}

/// Creates a new note and return it's name.
pub fn create(config: &Config) -> NoteResult<String> {
  let mut name = String::from("Untitled");
  let mut path = get_note_path(&config, &name)?;
  let mut n = 0;
  while path.as_path().exists() {
    n += 1;
    name = format!("Untitled {}", n);
    path = get_note_path(&config, &name)?;
  }

  fs::create_dir(path.clone())?;
  let readme = Path::new(&path).join("README.md");
  fs::write(readme, format!("# {}\n", name))?;
  let tags = Path::new(&path).join("tags.txt");
  fs::write(tags, "")?;

  Ok(name)
}

/// Renames a note.
pub fn rename(config: &Config, old_name: &str, new_name: &str) -> NoteResult<()> {
  let old_path = get_note_path(&config, old_name)?;
  let new_path = get_note_path(&config, new_name)?;
  Ok(fs::rename(old_path, new_path)?)
}

/// Writes the content of the given notes.
pub fn write(config: &Config, name: &str, content: &str) -> NoteResult<()> {
  let readme = Path::new(&get_note_path(&config, name)?).join("README.md");
  fs::write(readme, content.as_bytes())?;
  Ok(())
}

/// Removes the given note.
pub fn remove(config: &Config, name: &str) -> NoteResult<()> {
  let path = get_note_path(&config, name)?;
  Ok(fs::remove_dir_all(path)?)
}

/// Reads the tags associated with the given note.
pub fn read_tags(config: &Config, name: &str) -> NoteResult<Vec<String>> {
  let tags = Path::new(&get_note_path(&config, name)?).join("tags.txt");
  let content = fs::read_to_string(tags)?;
  let tags: Vec<String> = content
    .lines()
    .filter(|s| !s.is_empty())
    .map(String::from)
    .collect();
  Ok (tags)
}

/// Replaces the tags of tags of the given note.
pub fn write_tags(config: &Config, name: &str, tags: &Vec<String>) -> NoteResult<()> {
  let content = tags.join("\n");
  let tags = Path::new(&get_note_path(&config, name)?).join("tags.txt");
  fs::write(tags, content.as_bytes())?;
  Ok(())
}

pub fn take_screenshot(config: &Config, name: &str) -> NoteResult<String> {
  let id = Uuid::new_v4();
  let filename = format!("screenshot_{}.png", id);
  let path = Path::new(&get_note_path(&config, name)?).join(filename.clone());

  let (cmd, args) = config.get_screenshot_command(&path.to_string_lossy());

  let status = Command::new(cmd)
    .args(args)
    .status()?;
  
  match status.code() {
    Some(0) => Ok(filename),
    _ => Err("failed to take screenshot".into())
  }      
}

pub fn read_attachment(config: &Config, name: &str, attachment: &str, data: &mut Vec<u8>) -> NoteResult<()> {
  let path = Path::new(&get_note_path(&config, name)?).join(attachment);
  let mut file = File::open(path)?;
  file.read_to_end(data)?;
  Ok(())
}

pub fn open_note_directory(config: &Config, name: &str) -> NoteResult<()> {
  let path = get_note_path(&config, name)?;
  opener::open(path.as_path())?;
  Ok(())
}

fn get_note_path(config: &Config, name: &str) -> NoteResult<PathBuf> {
  check_name(name)?;
  let mut path = config.get_base_path();
  path.push(name);
  Ok(path)
}

/// Verifies, that a note name is valid
fn check_name(name: &str) -> NoteResult<()> {
  match name.chars().nth(0) {
    None | Some('/') | Some('\\') => Err(NoteError::new("invalid note name")),
    _ => Ok(()) 
  }
}

/// list all directories in `base_path` that have a `README.md` file inside
fn get_note_names(base_path: PathBuf) -> NoteResult<Vec<String>> {
  let mut note_names = Vec::<String>::new();
  let dir_entries = std::fs::read_dir(base_path)?;
  for dir_entry in dir_entries {
    match get_note_name(dir_entry?) {
      Ok(note_name) => note_names.push(note_name),
      Err(e) => error_handling(e.to_string())
    }
  }
  Ok(note_names)
}
  
/// check if the `dir_entry` contains a `README.md` file inside and returns the folder name if so
fn get_note_name(dir_entry: DirEntry) -> NoteResult<String> {
  if dir_entry.file_type()?.is_dir() && Path::new(&dir_entry.path()).join("README.md").is_file() {
    return Ok(dir_entry.file_name().into_string().unwrap());
  }

  Err(NoteError::new(&format!("README.md not found for `{:?}`.", dir_entry.path())))
}

fn error_handling(e: String) {
    // implementation up to discussion
    eprintln!("Warning: {e}");
}

