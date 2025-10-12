use std::fs::{self, File};
use std::io::Write;

use crate::app::config;
use crate::domain::app;

#[derive(Debug)]
pub enum ProjectDataError {
    Io(std::io::Error),
    JSON(serde_json::Error),
}

pub fn add_app(app: app::App, config: &config::Config) -> Result<(), ProjectDataError> {
    let mut apps = get_apps(config)?;

    apps.push(app);

    write_data_file(&apps, &config).map_err(|e| ProjectDataError::Io(e))?;

    Ok(())
}

pub fn get_app(
    app_name: &str,
    config: &config::Config,
) -> Result<Option<app::App>, ProjectDataError> {
    let apps = get_apps(config)?;

    Ok(apps
        .into_iter()
        .find(|a| a.name.to_lowercase() == app_name.to_lowercase()))
}

pub fn get_apps(config: &config::Config) -> Result<Vec<app::App>, ProjectDataError> {
    let apps_file_path = config.app_data_path.join("apps.json");
    let apps = match fs::read_to_string(&apps_file_path) {
        Ok(contents) => {
            let apps: Vec<app::App> =
                serde_json::from_str(&contents).map_err(|e| ProjectDataError::JSON(e))?;
            apps
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Vec::new(),
        Err(e) => panic!("Error getting app data: {}", e),
    };
    Ok(apps)
}

pub fn update_app(
    app_name: &str,
    app: &app::App,
    config: &config::Config,
) -> Result<(), ProjectDataError> {
    let apps = get_apps(config)?;
    let apps = apps
        .into_iter()
        .map(|a| if &a.name == app_name { app.clone() } else { a })
        .collect();

    write_data_file(&apps, &config).map_err(|e| ProjectDataError::Io(e))?;

    Ok(())
}

pub fn remove_app(app_name: &str, config: &config::Config) -> Result<(), ProjectDataError> {
    let apps = get_apps(config)?;

    let apps: Vec<app::App> = apps.into_iter().filter(|a| a.name != app_name).collect();

    write_data_file(&apps, config).map_err(|e| ProjectDataError::Io(e))?;

    Ok(())
}

fn write_data_file(apps: &Vec<app::App>, config: &config::Config) -> Result<(), std::io::Error> {
    let apps_json = serde_json::to_string_pretty(&apps)?;

    fs::create_dir_all(&config.app_data_path)?;

    let apps_file_path = config.app_data_path.join("apps.json");
    let mut file = File::create(&apps_file_path)?;
    file.write_all(apps_json.as_bytes())?;

    Ok(())
}

pub fn set_browser_path(path: &str, config: &config::Config) -> Result<(), ProjectDataError> {
    let mut new_config = config.clone();
    new_config.browser_path = Some(path.to_string());

    let apps = get_apps(&new_config)?;

    write_data_file(&apps, &new_config).map_err(|e| ProjectDataError::Io(e))?;

    Ok(())
}

pub fn get_browser_path(config: &config::Config) -> Option<String> {
    config.browser_path.clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::app::App;
    use crate::test::create_test_config::create_test_config;
    use std::fs;

    #[test]
    fn test_add_app() {
        let (config, _dir, _desktop_dir) = create_test_config();
        let app = App {
            name: "Test App".to_string(),
            url: "https://example.com".to_string(),
            icon: None,
        };
        let result = add_app(app, &config);
        assert!(result.is_ok());

        let apps_file_path = config.app_data_path.join("apps.json");
        let contents = fs::read_to_string(&apps_file_path).unwrap();
        let expected = "[\n  {\n    \"name\": \"Test App\",\n    \"url\": \"https://example.com\",\n    \"icon\": null\n  }\n]";
        assert_eq!(contents, expected);
    }

    #[test]
    fn test_get_app() {
        let (config, _dir, _desktop_dir) = create_test_config();
        let app = App {
            name: "Test App".to_string(),
            url: "https://example.com".to_string(),
            icon: None,
        };
        add_app(app, &config).unwrap();

        let app = get_app("Test App", &config).unwrap();
        assert!(app.is_some());

        let app = app.unwrap();
        assert_eq!(app.name, "Test App");
        assert_eq!(app.url, "https://example.com");
    }

    #[test]
    fn test_get_app_not_found() {
        let (config, _dir, _desktop) = create_test_config();

        let app = get_app("Test App", &config).unwrap();
        assert!(app.is_none());
    }

    #[test]
    fn test_get_apps() {
        let (config, _dir, _desktop) = create_test_config();
        let app = App {
            name: "Test App".to_string(),
            url: "https://example.com".to_string(),
            icon: None,
        };
        add_app(app.clone(), &config).unwrap();

        let apps = get_apps(&config).unwrap();
        assert_eq!(apps.len(), 1);
        assert_eq!(apps[0].name, app.name);
        assert_eq!(apps[0].url, app.url);
    }

    #[test]
    fn test_update_app() {
        let (config, _dir, _desktop) = create_test_config();
        let app = App {
            name: "Test App".to_string(),
            url: "https://example.com".to_string(),
            icon: None,
        };

        add_app(app, &config).unwrap();

        let new_app = App {
            name: "Test App new".to_string(),
            url: "https://example2.com".to_string(),
            icon: None,
        };
        update_app("Test App", &new_app, &config).unwrap();

        let apps = get_apps(&config).unwrap();
        assert_eq!(apps.len(), 1);
        assert_eq!(apps[0].name, "Test App new");
        assert_eq!(apps[0].url, "https://example2.com");
    }

    #[test]
    fn test_remove_app() {
        let (config, _dir, _desktop) = create_test_config();
        let app = App {
            name: "Test App".to_string(),
            url: "https://example.com".to_string(),
            icon: None,
        };

        add_app(app, &config).unwrap();

        let apps = get_apps(&config).unwrap();
        assert_eq!(apps.len(), 1);

        remove_app("Test App", &config).unwrap();

        let apps = get_apps(&config).unwrap();
        assert_eq!(apps.len(), 0);
    }
}
