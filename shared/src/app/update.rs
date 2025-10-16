use crate::app::config;
use crate::infra::app_data;
use crate::infra::desktop_data;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct UpdateOptions {
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug)]
pub enum UpdateError {
    AppNotFound,
    Io(std::io::Error),
}

pub fn update(
    name: &str,
    options: &UpdateOptions,
    config: &config::Config,
) -> Result<(), UpdateError> {
    let app = app_data::get_app(name, config).map_err(|e| {
        UpdateError::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("{:?}", e),
        ))
    })?;

    let mut app = match app {
        Some(app) => app,
        None => return Err(UpdateError::AppNotFound),
    };

    if let Some(name) = options.name.as_ref() {
        app.name = name.clone();
    }

    if let Some(url) = options.url.as_ref() {
        app.url = url.clone();
    }

    app_data::update_app(&name, &app, config).map_err(|e| {
        UpdateError::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("{:?}", e),
        ))
    })?;
    desktop_data::update_entry(&app, config).map_err(|e| {
        UpdateError::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("{:?}", e),
        ))
    })?;

    Ok(())
}
