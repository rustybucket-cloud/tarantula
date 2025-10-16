use crate::app::config::Config;
use crate::domain::app::App;
use crate::infra::app_data;
use crate::infra::desktop_data;
use crate::infra::icons;

#[derive(Debug)]
pub enum InstallError {
    Io(std::io::Error),
    Desktop(desktop_data::DesktopDataError),
    AppData(app_data::ProjectDataError),
    InvalidData(String),
}

pub fn install(name: &str, url: &str, config: &Config) -> Result<(), InstallError> {
    if ["install", "uninstall", "update", "list"].contains(&name.to_lowercase().as_str()) {
        return Err(InstallError::InvalidData(
            "App name cannot be a reserved word (install, uninstall, update, list)".to_string(),
        ));
    }

    let mut app = App {
        name: name.to_string(),
        url: url.to_string(),
        icon: None,
    };

    match icons::store_icon(&app, &config) {
        Ok(icon_url) => match icon_url {
            Some(u) => app.icon = Some(u),
            None => {}
        },
        Err(e) => eprintln!("{:?}", e),
    }

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
