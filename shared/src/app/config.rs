use std::path::PathBuf;

use crate::infra::config_data;
use crate::infra::config_data::ConfigDataError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub app_data_path: PathBuf,
    pub desktop_data_path: PathBuf,
    pub browser_path: Option<String>,
}

impl Config {
    pub fn new(app_data_path: PathBuf, desktop_data_path: PathBuf) -> Self {
        Self {
            app_data_path,
            desktop_data_path,
            browser_path: None,
        }
    }
}

#[derive(Debug)]
pub enum ConfigError {
    InvalidPath(String),
    JSON(serde_json::Error),
    Io(std::io::Error),
}

pub fn create_config() -> Result<Config, ConfigError> {
    let home_dir = dirs::home_dir()
        .ok_or("Could not find home directory")
        .map_err(|e| {
            ConfigError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                e.to_string(),
            ))
        })?;
    let app_data_path = home_dir.join(".local/share/tarantula").to_path_buf();
    let desktop_data_path = home_dir.join(".local/share/applications").to_path_buf();
    let mut config = Config::new(app_data_path, desktop_data_path);
    config.browser_path = match get_browser_path(&config) {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Error retrieving browser path from config: {:?}", e);
            std::process::exit(1);
        }
    };

    Ok(config)
}

pub fn update_browser_path(
    new_path: &str,
    config: &mut crate::app::config::Config,
) -> Result<(), ConfigError> {
    config.browser_path = Some(new_path.to_string());

    if std::path::Path::new(new_path).exists() == false {
        return Err(ConfigError::InvalidPath(
            "Executable path does not exist".to_string(),
        ));
    }

    config_data::update_browser_path(new_path, config).map_err(|e| match e {
        ConfigDataError::Io(e) => ConfigError::Io(e),
    })?;

    Ok(())
}

pub fn get_browser_path(
    config: &crate::app::config::Config,
) -> Result<Option<String>, ConfigError> {
    let path = config_data::get_browser_path(config).map_err(|e| ConfigError::Io(e))?;
    Ok(path)
}
