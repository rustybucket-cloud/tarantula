use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub app_data_path: PathBuf,
    pub desktop_data_path: PathBuf,
}

impl Config {
    pub fn new(app_data_path: PathBuf, desktop_data_path: PathBuf) -> Self {
        Self {
            app_data_path,
            desktop_data_path,
        }
    }
}
