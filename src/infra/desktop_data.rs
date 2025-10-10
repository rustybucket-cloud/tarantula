use crate::app::config::Config;
use crate::domain::app::App;
use std::fs::File;
use std::io::Write;

#[derive(Debug)]
pub enum DesktopDataError {
    Io(std::io::Error),
}

pub fn create_entry(app: &App, config: &Config) -> Result<(), DesktopDataError> {
    let file_name = app.name.to_lowercase().replace(' ', "_");
    let file_name = format!("{}.desktop", file_name);
    let path = config.desktop_data_path.clone().join(file_name);
    let mut file = File::create(path).map_err(DesktopDataError::Io)?;

    let mut content = format!(
        "[Desktop Entry]\nName={}\nExec=tarantula {}\nType=Application\n",
        app.name, app.name
    );
    if app.icon.is_some() {
        content.push_str(&format!("Icon={}\n", app.icon.as_ref().unwrap()));
    }

    file.write_all(content.as_bytes())
        .map_err(DesktopDataError::Io)?;

    Ok(())
}

pub fn update_entry(app: &App, config: &Config) -> Result<(), DesktopDataError> {
    // For simplicity, we'll just recreate the entry
    create_entry(app, config)
}

pub fn remove_entry(app_name: &str, config: &Config) -> Result<(), DesktopDataError> {
    let file_name = app_name.to_lowercase().replace(' ', "_");
    let file_name = format!("{}.desktop", file_name);
    let path = config.desktop_data_path.clone().join(file_name);
    std::fs::remove_file(path).map_err(DesktopDataError::Io)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::create_test_config::create_test_config;

    #[test]
    fn test_create_entry() {
        let (config, _app_dir, _desktop_dir) = create_test_config();
        let app = App {
            name: "Test App".to_string(),
            url: "http://example.com".to_string(),
            icon: Some("test_icon".to_string()),
        };
        create_entry(&app, &config).unwrap();
        let file_name = "test_app.desktop";
        let path = config.desktop_data_path.join(file_name);
        assert!(path.exists());

        let file = std::fs::read_to_string(path).unwrap();
        let expected_content = "[Desktop Entry]\nName=Test App\nExec=tarantula Test App\nType=Application\nIcon=test_icon\n";
        assert_eq!(file, expected_content);
    }
}
