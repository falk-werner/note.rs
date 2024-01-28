use std::{fs::DirEntry, path::{Path, PathBuf}};
use crate::config::{Config};
use crate::noteerror::{NoteError, NoteResult};

pub struct Note {
    config: Config
}

impl Note {

    /// Creates a new Note struct.
    pub fn new(config: Config) -> Self {
        Note { config }
    }


    /// list the names of the notes
    /// 
    /// list all directories in the base directory that have a `README.md` file inside
    pub fn list(&self) -> Result<Vec<String>, NoteError> {
        let base_path = self.config.get_base_path().join(".notes");
        match get_note_names(base_path) {
          Ok(note_names) => return Ok(note_names),
          Err(e) => error_handling(e.to_string())
        }
        Ok(Vec::<String>::new())
    }

    pub fn read(&self, name : &str) -> NoteResult<String> {
      Ok("Dummy content".into())
    }

    pub fn create(&self) -> NoteResult<String> {
      Ok("New Note".into())
    }

    pub fn rename(&self, old_name: &str, new_name: &str) -> NoteResult<()> {
      Err(NoteError::new("note implemented"))
    }

    pub fn write(&self, name: &str, content: &str) -> NoteResult<()> {
      Err(NoteError::new("note implemented"))
    }

    pub fn remove(&self, name: &str) -> NoteResult<()> {
      Err(NoteError::new("note implemented"))
    }

    pub fn read_tags(&self, name: &str) -> NoteResult<Vec<String>> {
      let tags: Vec<String> = Vec::new();
      Ok(tags) 
    }

    pub fn write_tags(&self, name: &str, tags: &Vec<String>) -> NoteResult<()> {
      Err(NoteError::new("note implemented"))
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

