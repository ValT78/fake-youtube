use chrono::Utc;

use crate::models::{PlaylistItemRecord, PlaylistSummary, VideoRecord};
use crate::services::youtube::YoutubePlaylistBundle;

#[derive(Debug, Clone)]
pub struct PlaylistImportPayload {
    pub playlist: PlaylistSummary,
    pub videos: Vec<VideoRecord>,
    pub playlist_items: Vec<PlaylistItemRecord>,
}

pub fn build_import_payload(source_url: &str, bundle: YoutubePlaylistBundle) -> PlaylistImportPayload {
    let synced_at = Utc::now().to_rfc3339();
    let playlist_id = format!("playlist:{}", bundle.playlist.youtube_playlist_id);
    let playlist = PlaylistSummary {
        id: playlist_id.clone(),
        youtube_playlist_id: bundle.playlist.youtube_playlist_id,
        title: bundle.playlist.title,
        description: bundle.playlist.description,
        channel_title: bundle.playlist.channel_title,
        thumbnail_url: bundle.playlist.thumbnail_url,
        video_count: if bundle.playlist.video_count > 0 {
            bundle.playlist.video_count
        } else {
            bundle.videos.len() as i64
        },
        source_url: source_url.to_string(),
        created_at: synced_at.clone(),
        updated_at: synced_at.clone(),
        last_synced_at: Some(synced_at.clone()),
    };

    let mut videos = Vec::with_capacity(bundle.videos.len());
    let mut playlist_items = Vec::with_capacity(bundle.videos.len());

    for video in bundle.videos {
        let video_id = format!("video:{}", video.youtube_video_id);

        videos.push(VideoRecord {
            id: video_id.clone(),
            youtube_video_id: video.youtube_video_id.clone(),
            title: video.title,
            description: video.description,
            channel_title: video.channel_title,
            thumbnail_url: video.thumbnail_url,
            published_at: video.published_at,
            duration_iso8601: video.duration_iso8601,
            duration_seconds: video.duration_seconds,
            created_at: synced_at.clone(),
            updated_at: synced_at.clone(),
        });

        playlist_items.push(PlaylistItemRecord {
            id: format!("playlist-item:{}:{}", playlist_id, video_id),
            playlist_id: playlist_id.clone(),
            video_id,
            youtube_playlist_item_id: video.youtube_playlist_item_id,
            position: video.position,
            created_at: synced_at.clone(),
        });
    }

    PlaylistImportPayload {
        playlist,
        videos,
        playlist_items,
    }
}

#[cfg(test)]
mod tests {
    use crate::services::youtube::{YoutubePlaylistBundle, YoutubePlaylistMetadata, YoutubePlaylistVideo};

    use super::build_import_payload;

    #[test]
    fn builds_consistent_import_payload() {
        let bundle = YoutubePlaylistBundle {
            playlist: YoutubePlaylistMetadata {
                youtube_playlist_id: "PLtest123456".to_string(),
                title: "Playlist test".to_string(),
                description: Some("Description".to_string()),
                channel_title: Some("Chaîne".to_string()),
                thumbnail_url: Some("https://img.example/playlist.jpg".to_string()),
                video_count: 1,
            },
            videos: vec![YoutubePlaylistVideo {
                youtube_video_id: "video42".to_string(),
                youtube_playlist_item_id: Some("item42".to_string()),
                title: "Vidéo 42".to_string(),
                description: None,
                channel_title: Some("Chaîne".to_string()),
                thumbnail_url: Some("https://img.example/video.jpg".to_string()),
                published_at: Some("2026-04-05T10:00:00Z".to_string()),
                duration_iso8601: Some("PT2M".to_string()),
                duration_seconds: Some(120),
                position: 0,
            }],
        };

        let payload = build_import_payload(
            "https://www.youtube.com/playlist?list=PLtest123456",
            bundle,
        );

        assert_eq!(payload.playlist.id, "playlist:PLtest123456");
        assert_eq!(payload.videos.len(), 1);
        assert_eq!(payload.playlist_items.len(), 1);
        assert_eq!(payload.playlist_items[0].video_id, "video:video42");
    }
}

