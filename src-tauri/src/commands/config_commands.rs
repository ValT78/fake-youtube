use tauri::AppHandle;

use crate::{
    errors::AppResult,
    services::app_config::{
        load_app_config,
        save_app_config as persist_app_config,
        AppConfig,
        AppConfigPayload,
    },
};

#[tauri::command]
pub fn get_app_config(app: AppHandle) -> AppResult<AppConfigPayload> {
    load_app_config(&app)
}

#[tauri::command]
pub fn save_app_config(app: AppHandle, config: AppConfig) -> AppResult<AppConfigPayload> {
    persist_app_config(&app, config)
}
