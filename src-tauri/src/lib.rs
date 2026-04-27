mod commands;
mod db;
mod errors;
mod models;
mod services;

use commands::config_commands::{get_app_config, save_app_config};
use commands::db_commands::database_status;
use commands::playlist_commands::{
    get_playlist_detail, import_playlist, list_playlists, open_video_in_vlc,
    parse_playlist_url,
};
use db::{migrations, DB_PLUGIN_URL};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default()
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations(DB_PLUGIN_URL, migrations())
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            get_app_config,
            save_app_config,
            parse_playlist_url,
            import_playlist,
            list_playlists,
            get_playlist_detail,
            open_video_in_vlc,
            database_status,
        ]);

    if let Err(error) = builder.run(tauri::generate_context!()) {
        eprintln!("failed to run playlist-browser: {error}");
    }
}
