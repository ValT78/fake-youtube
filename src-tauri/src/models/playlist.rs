use serde::Serialize;
use sqlx::FromRow;

use super::video::PlaylistVideoItem;

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistSummary {
    pub id: String,
    pub youtube_playlist_id: String,
    pub title: String,
    pub description: Option<String>,
    pub channel_title: Option<String>,
    pub thumbnail_url: Option<String>,
    pub video_count: i64,
    pub source_url: String,
    pub created_at: String,
    pub updated_at: String,
    pub last_synced_at: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistDetail {
    #[serde(flatten)]
    pub playlist: PlaylistSummary,
    pub videos: Vec<PlaylistVideoItem>,
}

#[derive(Debug, Clone)]
pub struct PlaylistItemRecord {
    pub id: String,
    pub playlist_id: String,
    pub video_id: String,
    pub youtube_playlist_item_id: Option<String>,
    pub position: i64,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportPlaylistResult {
    pub playlist: PlaylistSummary,
    pub imported_video_count: usize,
    pub source: &'static str,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedPlaylistUrl {
    pub original_url: String,
    pub canonical_url: String,
    pub playlist_id: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseStatus {
    pub ready: bool,
    pub database_path: String,
}

