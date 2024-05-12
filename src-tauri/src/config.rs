use serde::{Serialize, Deserialize};
use home::{home_dir};

use std::fs;
use std::path::{Path, PathBuf};

use crate::noteerror::{NoteError,NoteResult};

const DEFAULT_CONFIG_FILENAME: &str = ".noters.yaml";

pub struct Config {
    filename: Box<Path>,
    config_file: ConfigFile
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigFile {
    meta: MetaInfo,
    values: ConfigValues
}

#[derive(Serialize, Deserialize, Debug)]
struct MetaInfo {
    version: u32
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigValues {
    base_dir: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigValue {
    pub id: String,
    pub name: String,
    pub data_type: String,
    pub value: String
}

impl ConfigValue {
    fn new(id: &str, name: &str, data_type: &str, value: &str) -> Self {
        ConfigValue {
            id: String::from(id),
            name: String::from(name),
            data_type: String::from(data_type),
            value: String::from(value),
        }
    }
}


fn get_home_dir() -> PathBuf {
    home_dir().unwrap_or(PathBuf::from("."))
}

impl Config {

    /// Creates a new config with default values.
    fn new(filename: &Path) -> Self {
        Config {
            filename: filename.into(),
            config_file: { 
                ConfigFile {
                    meta: {
                        MetaInfo { 
                            version: 1
                        }
                    },
                    values: { 
                        ConfigValues {
                            base_dir: String::from("{home}/.notes")
                        }
                    }
                }
            }
        }
    }

    /// Loads the config from the default config file.
    ///
    /// The default config file is located at ~/.noters.yaml.
    /// When the file does not exist, a default config is created and stored
    /// at the default config file location.
    pub fn from_default_file() -> Self {
        let filename = get_home_dir().join(DEFAULT_CONFIG_FILENAME);

        match Config::from_file(filename.as_path()) {
            Ok(config) => config,
            Err(error) => {
                eprintln!("warning: failed to load config {:?}: {}", filename, error.to_string());
                let config = Config::new(filename.as_path());
                config.save();
                config
            }
        }
    }

    /// Try to load the config from a given path.
    fn from_file(filename: &Path) -> NoteResult<Self> {
        let text = fs::read_to_string(filename)?;
        let config_file = serde_yaml::from_str::<ConfigFile>(&text)?;
        
        if config_file.meta.version != 1 {
            return Err(NoteError::new("unknown file format (version mismatch)"));
        }

        Ok(Config { 
            filename: filename.into(),
            config_file
        })
    }

    /// Returns the base path, where the notes are stored.
    pub fn get_base_path(&self) -> PathBuf {
        let home_dir = get_home_dir();
        let base_dir = &self.config_file.values.base_dir;

        PathBuf::from(base_dir.replace("{home}", &home_dir.to_string_lossy()))
    }

    pub fn read_all(&self) -> Vec<ConfigValue> {
        vec!(
            ConfigValue::new("notes.path", "Notes Directory","string", &self.config_file.values.base_dir),
            ConfigValue::new("screenshot.command", "Screenshot Command","string", "gnome-screenshot -a {filename}"),
            ConfigValue::new("view.titlebar.color", "Titlebar Color","color", "#b0b0b0"),
        )
    }

    pub fn write(&mut self, _name: &str, _value: &str) {
        self.save();
    }

    /// Tries to save the configuration.
    ///
    /// Note that this function will not fail, even when the config file is not written.
    /// This behavior is intended, since there is no good way to handle those errors.
    /// At least, an error message is emitted.
    fn save(&self) {
        let text = serde_yaml::to_string(&self.config_file).unwrap();
        if let Err(error) = fs::write(self.filename.as_ref(), text.as_bytes()) {
            eprintln!("error: failed to save config: {:?}: {}", self.filename, error);
        }
        else {
            eprintln!("info: saved config: {:?}", self.filename);
        }
    }
}

#[cfg(test)]
mod tests {

use crate::Config;
use crate::config::{get_home_dir};
use tempfile::tempdir;
use std::fs;
use std::path::{PathBuf};

#[test]
fn create_default_config() {
    let path = PathBuf::from("/test/config.yaml");
    let config = Config::new(path.as_path());

    assert_eq!(1, config.config_file.meta.version);
    assert_eq!(*path, *config.filename);
    assert_eq!("{home}/.notes", config.config_file.values.base_dir);
}

#[test]
fn get_base_path_resolves_home() {
    let path = PathBuf::from("/test/config.yaml");
    let config = Config::new(path.as_path());

    let expected = get_home_dir().join(".notes");
    let actual = config.get_base_path();

    assert_eq!(expected, actual);
}

#[test]
fn load_config_from_file() {
    let text = br##"meta:
  version: 1
values:
  base_dir: '/path/to/notes'"##;

    let temp_dir = tempdir().unwrap();
    let config_path = temp_dir.path().join("config.yaml");
    fs::write(config_path.clone(), text).unwrap();

    let config = Config::from_file(config_path.as_path()).unwrap();
    assert_eq!("/path/to/notes", config.config_file.values.base_dir);

    let _ = temp_dir.close();
}

#[test]
fn fail_to_load_non_existing_config_file() {
    let temp_dir = tempdir().unwrap();
    let config_path = temp_dir.path().join("config.yaml");

    let config = Config::from_file(config_path.as_path());
    assert!(config.is_err());

    let _ = temp_dir.close();
}

#[test]
fn fail_to_load_invalid_config_file() {
    let text = br##"\t this is not yaml"##;

    let temp_dir = tempdir().unwrap();
    let config_path = temp_dir.path().join("config.yaml");
    fs::write(config_path.clone(), text).unwrap();

    let config = Config::from_file(config_path.as_path());
    assert!(config.is_err());

    let _ = temp_dir.close();
}

#[test]
fn fail_to_load_config_file_with_wrong_version() {
    let text = br##"meta:
  version: 0
values:
  base_dir: '/path/to/notes'"##;

    let temp_dir = tempdir().unwrap();
    let config_path = temp_dir.path().join("config.yaml");
    fs::write(config_path.clone(), text).unwrap();

    let config = Config::from_file(config_path.as_path());
    assert!(config.is_err());

    let _ = temp_dir.close();
}

#[test]
fn save_config_file() {
    let temp_dir = tempdir().unwrap();
    let config_path = temp_dir.path().join("config.yaml");

    let config = Config::new(config_path.as_path());
    config.save();

    let config = Config::from_file(config_path.as_path());
    assert!(config.is_ok());

    let _ = temp_dir.close();
}

#[test]
fn save_does_not_fail_if_config_file_cannot_be_written() {
    let temp_dir = tempdir().unwrap();
    let config_path = temp_dir.path().join("non-existing-dir").join("config.yaml");

    let config = Config::new(config_path.as_path());
    config.save();

    let config = Config::from_file(config_path.as_path());
    assert!(config.is_err());

    let _ = temp_dir.close();
}


}