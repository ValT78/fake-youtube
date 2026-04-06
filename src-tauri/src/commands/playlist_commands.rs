use tauri::AppHandle;

use crate::{
    db,
    errors::AppResult,
    models::{ImportPlaylistResult, ParsedPlaylistUrl, PlaylistDetail, PlaylistSummary},
    services::{
        player_launcher::launch_video_in_vlc,
        playlist_parser::parse_playlist_reference,
        sync::build_import_payload,
        youtube::YoutubeService,
    },
};

#[tauri::command]
pub fn parse_playlist_url(source_url: String) -> AppResult<ParsedPlaylistUrl> {
    parse_playlist_reference(&source_url)
}

#[tauri::command]
pub async fn import_playlist(app: AppHandle, source_url: String) -> AppResult<ImportPlaylistResult> {
    let parsed = parse_playlist_reference(&source_url)?;
    let youtube = YoutubeService::new_from_env()?;
    let playlist_bundle = youtube.fetch_playlist_bundle(&parsed.playlist_id).await?;
    let payload = build_import_payload(&parsed.canonical_url, playlist_bundle);
    let playlist = payload.playlist.clone();
    let imported_video_count = payload.videos.len();

    db::persist_playlist_import(&app, &payload).await?;

    Ok(ImportPlaylistResult {
        playlist,
        imported_video_count,
        source: "api",
    })
}

#[tauri::command]
pub async fn list_playlists(app: AppHandle) -> AppResult<Vec<PlaylistSummary>> {
    db::list_playlists(&app).await
}

#[tauri::command]
pub async fn get_playlist_detail(app: AppHandle, playlist_id: String) -> AppResult<PlaylistDetail> {
    db::get_playlist_detail(&app, &playlist_id).await
}

#[tauri::command]
pub fn open_video_in_vlc(video_id: String) -> AppResult<()> {
    launch_video_in_vlc(&video_id)
}
