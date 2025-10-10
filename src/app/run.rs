use crate::app::config;
use crate::infra::app_data;
use std::process::Command;

pub enum RunError {
    AppNotFound(String),
    LaunchFailed(String),
    Io(std::io::Error),
}

pub fn run(app_name: &str, config: &config::Config) -> Result<(), RunError> {
    let app = app_data::get_app(app_name, config)
        .map_err(|e| {
            RunError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("{:?}", e),
            ))
        })?
        .ok_or_else(|| RunError::AppNotFound(app_name.to_string()))?;

    let output = Command::new("/usr/bin/chromium")
        .arg(format!("--app={}", app.url))
        .output()
        .map_err(RunError::Io)?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Output:\n{}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error:\n{}", stderr);
    }

    Ok(())
}
