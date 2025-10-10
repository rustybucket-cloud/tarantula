use crate::app::config;
use crate::infra::app_data;
use exec;

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

    let error = exec::Command::new("/usr/bin/chromium")
        .arg(format!("--app={}", app.url))
        .exec();

    return Err(RunError::LaunchFailed(error.to_string()));
}
