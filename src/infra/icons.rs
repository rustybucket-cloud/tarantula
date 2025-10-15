use crate::app::config;
use crate::domain::app::App;
use std::fs;
use std::path::Path;
use url::Url;

#[derive(Debug)]
pub enum StoreIconError {
    Reqwest(reqwest::Error),
    Io(std::io::Error),
}

pub fn store_icon(app: &App, config: &config::Config) -> Result<Option<String>, StoreIconError> {
    let url = Url::parse(&app.url).unwrap();
    let url = url.host_str().unwrap();

    let bytes = match get_icon_data(url) {
        Some(b) => b,
        None => return Ok(None),
    };

    let path = Path::new(&config.app_data_path)
        .join(format!("{}.ico", app.name.to_lowercase().replace(" ", "_")));

    fs::write(&path, &bytes).map_err(|e| StoreIconError::Io(e))?;

    Ok(Some(path.to_str().unwrap().to_string()))
}

fn get_icon_data(url: &str) -> Option<Vec<u8>> {
    if let Some(bytes) = get_icon_from_url(format!("https://{}/favicon.ico", url).as_str()) {
        return Some(bytes);
    }

    if let Some(bytes) = get_icon_from_url(format!("https://{}/assets/favicon.ico", url).as_str()) {
        return Some(bytes);
    }

    if let Some(bytes) = get_icon_from_url(format!("https://{}/assets/favicon.png", url).as_str()) {
        return Some(bytes);
    }

    if let Some(bytes) = get_icon_from_url(format!("https://{}/images/favicon.ico", url).as_str()) {
        return Some(bytes);
    }

    if let Some(bytes) = get_icon_from_url(format!("https://{}/images/favicon.png", url).as_str()) {
        return Some(bytes);
    }

    None
}

fn get_icon_from_url(url: &str) -> Option<Vec<u8>> {
    let resp = match reqwest::blocking::get(url) {
        Ok(r) => r,
        Err(_) => return None,
    };
    let bytes = match resp.bytes() {
        Ok(b) => b,
        Err(e) => {
            eprintln!("{}", e);
            return None;
        }
    };

    Some(bytes.to_vec())
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use super::*;
    use crate::test::create_test_config::create_test_config;

    #[test]
    fn test_get_icon() {
        let (config, _a, _d) = create_test_config();

        let app = App {
            name: "Perplexity".to_string(),
            url: "https://www.perplexity.ai/".to_string(),
            icon: None,
        };

        let icon_path = store_icon(&app, &config).unwrap();
        assert_eq!(
            &icon_path,
            &format!("{}/perplexity.ico", config.app_data_path.to_str().unwrap())
        );

        assert!(Path::new(&icon_path).exists());
    }
}
