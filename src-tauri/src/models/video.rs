use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Clone)]
pub struct VideoRecord {
    pub id: String,
    pub youtube_video_id: String,
    pub title: String,
    pub description: Option<String>,
    pub channel_title: Option<String>,
    pub thumbnail_url: Option<String>,
    pub published_at: Option<String>,
    pub duration_iso8601: Option<String>,
    pub duration_seconds: Option<i64>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistVideoItem {
    pub id: String,
    pub youtube_video_id: String,
    pub youtube_playlist_item_id: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub channel_title: Option<String>,
    pub thumbnail_url: Option<String>,
    pub published_at: Option<String>,
    pub duration_iso8601: Option<String>,
    pub duration_seconds: Option<i64>,
    pub position: i64,
}

