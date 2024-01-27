use std::{fs::DirEntry, io::{Error, Result}, path::{Path, PathBuf}};
use crate::config::{Config};

pub struct Notebook {
    config: Config
}

impl Notebook {

    /// Creates a new Note struct.
    pub fn new(config: Config) -> Self {
        Notebook { config }
    }


    /// list the names of the notes
    /// 
    /// list all directories in the base directory that have a `README.md` file inside
    pub fn list_notes(&self) -> Vec<String> {
        let base_path = self.config.get_base_path().join(".notes");
        match get_note_names(base_path) {
          Ok(note_names) => return note_names,
          Err(e) => error_handling(e.to_string())
        }
        Vec::<String>::new()      
    }

}

/// list all directories in `base_path` that have a `README.md` file inside
fn get_note_names(base_path: PathBuf) -> Result<Vec<String>> {
  let mut note_names = Vec::<String>::new();
  let dir_entries = std::fs::read_dir(base_path)?;
  for dir_entry in dir_entries {
    match get_note_name(dir_entry) {
      Ok(note_name) => note_names.push(note_name),
      Err(e) => error_handling(e.to_string())
    }
  }
  Ok(note_names)
}
  
/// check if the `dir_entry` contains a `README.md` file inside and returns the folder name if so
fn get_note_name(dir_entry: Result<DirEntry>) -> Result<String> {
  let dir_entry = dir_entry?;
  if dir_entry.file_type()?.is_dir() && Path::new(&dir_entry.path()).join("README.md").is_file() {
    return Ok(dir_entry.file_name().into_string().unwrap());
  }
  Err(
    Error::new(
      std::io::ErrorKind::NotFound,
      format!("README.md not found for `{}`.", dir_entry.path().to_str().unwrap())
    )
  )
}

fn error_handling(e: String) {
    // implementation up to discussion
    eprintln!("Warning: {e}");
}

