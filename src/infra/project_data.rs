use std::fs::{self, File};
use std::io::Write;

use crate::app::config;
use crate::domain::app;

pub enum AddAppError {
    Io(std::io::Error),
    JSON(serde_json::Error),
}

pub fn add_app(app: app::App, config: &config::Config) -> Result<(), AddAppError> {
    let mut apps = match fs::read_to_string(&config.data_path) {
        Ok(contents) => {
            let apps: Vec<app::App> =
                serde_json::from_str(&contents).map_err(|e| AddAppError::JSON(e))?;
            apps
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Vec::new(),
        Err(e) => panic!("Error getting app data: {}", e),
    };

    apps.push(app);

    let apps_json = serde_json::to_string_pretty(&apps).expect("Failed to serialize app data");

    let mut file = File::create(&config.data_path).expect("Failed to update app data file");
    file.write_all(apps_json.as_bytes())
        .expect("Failed to update app data file");

    Ok(())
}
