use std::path::PathBuf;

use crate::infra::app_data;
use crate::infra::app_data::ProjectDataError;

#[derive(Debug, Clone)]
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

    app_data::set_browser_path(new_path, config).map_err(|e| match e {
        ProjectDataError::Io(io_err) => ConfigError::Io(io_err),
        ProjectDataError::JSON(json_err) => ConfigError::JSON(json_err),
    })?;

    Ok(())
}
