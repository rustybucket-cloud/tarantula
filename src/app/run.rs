use std::path::Path;

use crate::app::config;
use crate::infra::app_data::{self, ProjectDataError};
use crate::utils;
use exec;

#[derive(Debug)]
pub enum RunError {
    AppNotFound(String),
    LaunchFailed(String),
    Io(std::io::Error),
}

pub fn run(app_name: &str, config: &config::Config) -> Result<(), RunError> {
    let url = match app_data::get_app(app_name, config) {
        Ok(app) => match app {
            Some(a) => a.url,
            None => {
                let is_url = utils::is_url(app_name);
                if is_url {
                    app_name.to_string()
                } else {
                    return Err(RunError::AppNotFound(app_name.to_string()));
                }
            }
        },
        Err(ProjectDataError::Io(e)) => return Err(RunError::Io(e)),
        Err(ProjectDataError::JSON(e)) => {
            return Err(RunError::LaunchFailed(format!(
                "Failed to parse apps data: {}",
                e
            )));
        }
    };

    let browser_path = match &config.browser_path {
        Some(path) => path.clone(),
        None => match get_browser_path() {
            Some(path) => path,
            None => {
                return Err(RunError::LaunchFailed(
                    "Could not determine default browser".to_string(),
                ));
            }
        },
    };

    let error = exec::Command::new(browser_path)
        .arg(format!("--app={}", url))
        .exec();

    return Err(RunError::LaunchFailed(error.to_string()));
}

fn get_browser_path() -> Option<String> {
    let default_browser = match std::process::Command::new("xdg-settings")
        .arg("get")
        .arg("default-web-browser")
        .output()
        .ok()
    {
        Some(output) => String::from_utf8_lossy(&output.stdout).trim().to_string(),
        None => return None,
    };

    if Path::new("/usr/share/applications/")
        .join(&default_browser)
        .exists()
    {
        let exec_path =
            get_exec_from_desktop(&Path::new("/usr/share/applications/").join(&default_browser));
        if exec_path.is_some() {
            return exec_path;
        }
    }

    if Path::new("/etc/xdg/autostart/")
        .join(&default_browser)
        .exists()
    {
        let exec_path =
            get_exec_from_desktop(&Path::new("/etc/xdg/autostart/").join(&default_browser));
        if exec_path.is_some() {
            return exec_path;
        }
    }

    if Path::new("/usr/bin/").join(&default_browser).exists() {
        return Some("/usr/bin/".to_owned() + &default_browser);
    }

    if Path::new("/usr/local/bin/").join(&default_browser).exists() {
        return Some("/usr/local/bin/".to_owned() + &default_browser);
    }

    if Path::new("/snap/bin/").join(&default_browser).exists() {
        return Some("/snap/bin/".to_owned() + &default_browser);
    }

    None
}

fn get_exec_from_desktop(path: &Path) -> Option<String> {
    let file_content = std::fs::read_to_string(path).ok()?;
    for line in file_content.lines() {
        if line.starts_with("Exec=") {
            let exec_line = line.trim_start_matches("Exec=").trim();
            let exec_parts: Vec<&str> = exec_line.split_whitespace().collect();
            if !exec_parts.is_empty() {
                let exec_path = exec_parts[0];
                if exec_path.contains("%") {
                    return Some(exec_path.split('%').next().unwrap().to_string());
                } else {
                    return Some(exec_path.to_string());
                }
            }
        }
    }
    None
}
