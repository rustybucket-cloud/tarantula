use shared::app::config;
use shared::app::run;
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_app_data, run_app])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
