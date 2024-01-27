use serde::{Serialize, Deserialize};
use home::{home_dir};

use std::fs;
use std::path::{Path, PathBuf};
use core::fmt::Display;

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

fn get_home_dir() -> PathBuf {
    home_dir().unwrap_or(PathBuf::from("."))
}

struct ConfigError {
    message: String
}

impl ConfigError {
    fn new(message: &str) -> Self {
        ConfigError { message: message.into() }
    }
}

impl<E: Display> From<E> for ConfigError {
    fn from(value: E) -> Self {
        ConfigError { message: value.to_string() }
    }
}

impl Config {

    /// Creates a new config with default values.
    fn new() -> Self {
        Config {
            filename: get_home_dir().join(DEFAULT_CONFIG_FILENAME).as_path().into(),
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
                eprintln!("warning: failed to load config {:?}: {}", filename, error.message);
                let config = Config::new();
                config.save();
                config
            }
        }
    }

    /// Try to load the config from a given path.
    fn from_file(filename: &Path) -> Result<Self, ConfigError> {
        let text = fs::read_to_string(filename)?;
        let config_file = serde_yaml::from_str::<ConfigFile>(&text)?;
        
        if config_file.meta.version != 1 {
            return Err(ConfigError::new("unknown file format (version mismatch)"));
        }

        Ok(Config { 
            filename: filename.into(),
            config_file: config_file
        })
    }

    /// Returns the base path, where the notes are stored.
    pub fn get_base_path(&self) -> PathBuf {
        let home_dir = get_home_dir();
        let base_dir = &self.config_file.values.base_dir;

        PathBuf::from(base_dir.replace("{home}", &home_dir.to_string_lossy()))
    }

    /// Tries to save the configuration.
    ///
    /// Note that this function will not fail, even when the config file is not written.
    /// This behavior is intended, since there is no good way to handle those errors.
    /// At least, an error message is emitted.
    fn save(&self) {
        let text = serde_yaml::to_string(&self.config_file).unwrap();
        if let Err(error) = fs::write(self.filename.as_ref(), text.as_bytes()) {
            eprintln!("error: failed to save config: {:?}: {}", self.filename, error.to_string());
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

#[test]
fn create_default_config() {
    let config = Config::new();

    assert_eq!(1, config.config_file.meta.version);
    assert_eq!(".noters.yaml", config.filename.file_name().unwrap());
    assert_eq!("{home}/.notes", config.config_file.values.base_dir);
}

#[test]
fn get_base_path_resolves_home() {
    let config = Config::new();

    let expected = get_home_dir().join(".notes");
    let actual = config.get_base_path();

    assert_eq!(expected, actual);
}


}