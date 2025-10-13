use crate::app::config;
use std::io::Write;

pub enum ConfigDataError {
    Io(std::io::Error),
}

pub fn update_browser_path(
    new_path: &str,
    config: &mut config::Config,
) -> Result<(), ConfigDataError> {
    config.browser_path = Some(new_path.to_string());

    update_config_file(config).map_err(|e| ConfigDataError::Io(e))?;

    Ok(())
}

pub fn get_browser_path(config: &config::Config) -> Result<Option<String>, std::io::Error> {
    let file = match std::fs::File::open(config.app_data_path.join("config.json")) {
        Ok(f) => f,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            std::fs::create_dir_all(&config.app_data_path)?;
            let mut file = std::fs::File::create(config.app_data_path.join("config.json"))?;
            file.write_all(
                b"{\"app_data_path\":\"\",\"desktop_data_path\":\"\",\"browser_path\":null}",
            )?;

            std::fs::File::open(config.app_data_path.join("config.json"))?
        }
        Err(e) => return Err(e),
    };
    let reader = std::io::BufReader::new(file);
    let config: config::Config = serde_json::from_reader(reader)?;
    Ok(config.browser_path)
}

fn update_config_file(config: &config::Config) -> Result<(), std::io::Error> {
    let config_json = serde_json::to_string_pretty(&config).unwrap();

    std::fs::create_dir_all(&config.app_data_path)?;

    let config_file_path = config.app_data_path.join("config.json");
    let mut file = std::fs::File::create(&config_file_path)?;
    file.write_all(config_json.as_bytes())?;

    Ok(())
}
