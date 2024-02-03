use std::{fs, fs::DirEntry, path::{Path, PathBuf}};
use crate::config::{Config};
use crate::noteerror::{NoteError, NoteResult};


use opener;

pub struct Note {
    config: Config
}

impl Note {

    /// Creates a new Note struct.
    pub fn new(config: Config) -> Self {
        Note { config }
    }


    /// Lists the names of the notes.
    /// 
    /// list all directories in the base directory that have a `README.md` file inside
    pub fn list(&self) -> Result<Vec<String>, NoteError> {
        let base_path = self.config.get_base_path();
        match get_note_names(base_path) {
          Ok(note_names) => return Ok(note_names),
          Err(e) => error_handling(e.to_string())
        }
        Ok(Vec::<String>::new())
    }

    /// Reads the contents of the given note.
    pub fn read(&self, name : &str) -> NoteResult<String> {
      let readme = Path::new(&self.get_note_path(name)?).join("README.md");
      Ok(fs::read_to_string(readme)?)
    }

    /// Creates a new note and return it's name.
    pub fn create(&self) -> NoteResult<String> {
      let mut name = String::from("Untitled");
      let mut path = self.get_note_path(&name)?;
      let mut n = 0;
      while path.as_path().exists() {
        n += 1;
        name = format!("Untitled {}", n);
        path = self.get_note_path(&name)?;
      }

      fs::create_dir(path.clone())?;
      let readme = Path::new(&path).join("README.md");
      fs::write(readme, format!("# {}\n", name))?;
      let tags = Path::new(&path).join("tags.txt");
      fs::write(tags, "")?;

      Ok(name)
    }

    /// Renames a note.
    pub fn rename(&self, old_name: &str, new_name: &str) -> NoteResult<()> {
      let old_path = self.get_note_path(old_name)?;
      let new_path = self.get_note_path(new_name)?;
      Ok(fs::rename(old_path, new_path)?)
    }

    /// Writes the content of the given notes.
    pub fn write(&self, name: &str, content: &str) -> NoteResult<()> {
      let readme = Path::new(&self.get_note_path(name)?).join("README.md");
      fs::write(readme, content.as_bytes())?;
      Ok(())
    }

    /// Removes the given note.
    pub fn remove(&self, name: &str) -> NoteResult<()> {
      let path = self.get_note_path(name)?;
      Ok(fs::remove_dir_all(path)?)
    }

    /// Reads the tags associated with the given note.
    pub fn read_tags(&self, name: &str) -> NoteResult<Vec<String>> {
      let tags = Path::new(&self.get_note_path(name)?).join("tags.txt");
      let content = fs::read_to_string(tags)?;
      let tags: Vec<String> = content
        .lines()
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect();
      Ok (tags)
    }

    /// Replaces the tags of tags of the given note.
    pub fn write_tags(&self, name: &str, tags: &Vec<String>) -> NoteResult<()> {
      let content = tags.join("\n");
      let tags = Path::new(&self.get_note_path(name)?).join("tags.txt");
      fs::write(tags, content.as_bytes())?;
      Ok(())
    }

    fn get_note_path(&self, name: &str) -> NoteResult<PathBuf> {
      check_name(name)?;
      let mut path = self.config.get_base_path();
      path.push(name);
      Ok(path)
    }

    pub fn open_note_direcotry(&self, name: &str) -> NoteResult<()> {
      let path = self.get_note_path(name)?;
      opener::open(path.as_path())?;
      Ok(())
    }
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

