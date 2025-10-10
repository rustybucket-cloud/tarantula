use crate::app::config::Config;
use crate::infra::app_data;
use crate::infra::desktop_data::remove_entry;

pub enum UninstallError {
    AppNotFound,
    Io(std::io::Error),
}

pub fn uninstall(app_name: &str, config: &Config) -> Result<(), UninstallError> {
    app_data::remove_app(app_name, config).map_err(|e| {
        UninstallError::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("{:?}", e),
        ))
    })?;

    remove_entry(&app_name, config).map_err(|e| {
        UninstallError::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("{:?}", e),
        ))
    })?;

    Ok(())
}
