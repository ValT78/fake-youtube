use std::{
    env,
    fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

use crate::errors::{AppError, AppResult};

const CONFIG_FILE_NAME: &str = "playlist-browser.config.json";
const LEGACY_ENV_YOUTUBE_API_KEY: &str = "YOUTUBE_API_KEY";
const LEGACY_ENV_VLC_PATH: &str = "VLC_PATH";
const LEGACY_ENV_YTDLP_PATH: &str = "YTDLP_PATH";

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    #[serde(default)]
    pub youtube_api_key: String,
    #[serde(default)]
    pub vlc_path: String,
    #[serde(default)]
    pub ytdlp_path: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppConfigPayload {
    pub config: AppConfig,
    pub config_path: String,
    pub config_source: String,
}

impl AppConfig {
    pub fn youtube_api_key(&self) -> Option<&str> {
        normalized_value(&self.youtube_api_key)
    }

    pub fn vlc_path(&self) -> Option<&str> {
        normalized_value(&self.vlc_path)
    }

    pub fn ytdlp_path(&self) -> Option<&str> {
        normalized_value(&self.ytdlp_path)
    }
}

pub fn load_app_config(app: &AppHandle) -> AppResult<AppConfigPayload> {
    if let Some(exe_config_path) = executable_config_path() {
        if exe_config_path.exists() {
            let config = read_config_file(&exe_config_path)?;

            return Ok(AppConfigPayload {
                config,
                config_path: exe_config_path.display().to_string(),
                config_source: "executable".to_string(),
            });
        }
    }

    let app_config_path = app_config_path(app)?;

    if app_config_path.exists() {
        let config = read_config_file(&app_config_path)?;

        return Ok(AppConfigPayload {
            config,
            config_path: app_config_path.display().to_string(),
            config_source: "appData".to_string(),
        });
    }

    Ok(AppConfigPayload {
        config: load_legacy_env_fallback(),
        config_path: app_config_path.display().to_string(),
        config_source: "default".to_string(),
    })
}

pub fn save_app_config(app: &AppHandle, config: AppConfig) -> AppResult<AppConfigPayload> {
    let config_path = app_config_path(app)?;
    let parent_dir = config_path
        .parent()
        .ok_or_else(|| AppError::internal("Impossible de déterminer le dossier de configuration."))?;

    fs::create_dir_all(parent_dir).map_err(|_| AppError::ConfigWriteFailed)?;

    let serialized =
        serde_json::to_string_pretty(&config).map_err(|_| AppError::ConfigWriteFailed)?;

    fs::write(&config_path, serialized).map_err(|_| AppError::ConfigWriteFailed)?;

    Ok(AppConfigPayload {
        config,
        config_path: config_path.display().to_string(),
        config_source: "appData".to_string(),
    })
}

fn app_config_path(app: &AppHandle) -> AppResult<PathBuf> {
    let config_dir = app
        .path()
        .app_config_dir()
        .map_err(|_| AppError::ConfigPathUnavailable)?;

    Ok(config_dir.join(CONFIG_FILE_NAME))
}

fn executable_config_path() -> Option<PathBuf> {
    let executable_path = env::current_exe().ok()?;
    let executable_dir = executable_path.parent()?;
    Some(executable_dir.join(CONFIG_FILE_NAME))
}

fn read_config_file(path: &Path) -> AppResult<AppConfig> {
    let raw = fs::read_to_string(path).map_err(|_| AppError::ConfigReadFailed)?;
    serde_json::from_str(&raw).map_err(|_| AppError::ConfigReadFailed)
}

fn load_legacy_env_fallback() -> AppConfig {
    AppConfig {
        youtube_api_key: env::var(LEGACY_ENV_YOUTUBE_API_KEY).unwrap_or_default(),
        vlc_path: env::var(LEGACY_ENV_VLC_PATH).unwrap_or_default(),
        ytdlp_path: env::var(LEGACY_ENV_YTDLP_PATH).unwrap_or_default(),
    }
}

fn normalized_value(value: &str) -> Option<&str> {
    let trimmed = value.trim();

    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}
