use crate::app::config::Config;
use crate::infra::app_data;
use crate::infra::desktop_data::remove_entry;

#[derive(Debug)]
pub enum UninstallError {
    AppNotFound,
    Io(std::io::Error),
}

pub fn uninstall(app_name: &str, config: &Config) -> Result<(), UninstallError> {
    app_data::remove_app(app_name, config).map_err(|e| match e {
        app_data::ProjectDataError::Io(io_err) if io_err.kind() == std::io::ErrorKind::NotFound => {
            return UninstallError::AppNotFound;
        }
        e => {
            return UninstallError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("{:?}", e),
            ));
        }
    })?;

    remove_entry(&app_name, config).map_err(|e| {
        UninstallError::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("{:?}", e),
        ))
    })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::install;
    use std::path::Path;

    #[test]
    fn test_uninstall() {
        let (config, _dir, _desktop_dir) = crate::test::create_test_config::create_test_config();
        let name = "Test App";
        let url = "https://example.com";

        // First, install the app
        install::install(name, url, &config).unwrap();

        // Now, uninstall the app
        uninstall(name, &config).unwrap();

        // removes the app entry
        let apps = app_data::get_apps(&config).unwrap();
        assert_eq!(apps.len(), 0);

        // removes the desktop entry
        assert!(
            Path::new(&config.desktop_data_path)
                .join("test_app.desktop")
                .exists()
                == false
        );
    }
}
