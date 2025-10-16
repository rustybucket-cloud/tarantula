use crate::app::config;

// we need to return the TempDir to keep it alive during the test
pub fn create_test_config() -> (config::Config, tempfile::TempDir, tempfile::TempDir) {
    let app_dir = tempfile::tempdir().unwrap();
    let app_data_path = app_dir.path().to_path_buf();

    let desktop_dir = tempfile::tempdir().unwrap();
    let desktop_data_path = desktop_dir.path().to_path_buf();

    (
        config::Config {
            app_data_path,
            desktop_data_path,
            browser_path: None,
        },
        app_dir,
        desktop_dir,
    )
}
