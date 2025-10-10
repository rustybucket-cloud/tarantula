use crate::app::config::Config;
use crate::domain::app::App;
use crate::infra::app_data;
use crate::infra::desktop_data;

#[derive(Debug)]
pub enum InstallError {
    Io(std::io::Error),
    Desktop(desktop_data::DesktopDataError),
    AppData(app_data::ProjectDataError),
}

pub fn install(name: &str, url: &str, config: &Config) -> Result<(), InstallError> {
    let app = App {
        name: name.to_string(),
        url: url.to_string(),
        icon: None,
    };

    app_data::add_app(app.clone(), config).map_err(|e| InstallError::AppData(e))?;
    desktop_data::create_entry(&app, config).map_err(|e| InstallError::Desktop(e))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_install() {
        let (config, _dir, _desktop_dir) = crate::test::create_test_config::create_test_config();
        let name = "Test App";
        let url = "https://example.com";

        install(name, url, &config).unwrap();

        let apps = app_data::get_apps(&config).unwrap();
        assert_eq!(apps.len(), 1);
        assert_eq!(apps[0].name, name);
        assert_eq!(apps[0].url, url);
    }
}
