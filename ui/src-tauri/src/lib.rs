use shared::app::config;
use shared::app::install;
use shared::app::run;
use shared::app::uninstall;
use shared::app::update;
use shared::domain::app::App;
use shared::infra::app_data;

#[tauri::command]
fn get_app_data() -> Vec<App> {
    let config = config::create_config().unwrap();
    app_data::get_apps(&config).unwrap()
}

#[tauri::command]
fn run_app(app_name: String) {
    let config = config::create_config().unwrap();
    run::run(&app_name, &config).unwrap();
}

#[tauri::command]
fn install_app(name: String, url: String) -> Result<(), String> {
    let config = config::create_config().unwrap();
    install::install(&name, &url, &config).map_err(|e| format!("{:?}", e))
}

#[tauri::command]
fn uninstall_app(name: String) -> Result<(), String> {
    let config = config::create_config().unwrap();
    uninstall::uninstall(&name, &config).map_err(|e| format!("{:?}", e))
}

#[tauri::command]
fn update_app(name: String, options: update::UpdateOptions) -> Result<(), String> {
    let config = config::create_config().unwrap();
    update::update(&name, &options, &config).map_err(|e| format!("{:?}", e))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_app_data,
            run_app,
            install_app,
            uninstall_app,
            update_app
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
