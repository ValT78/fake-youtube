use std::collections::{HashMap, HashSet};

use reqwest::{Client, StatusCode};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use tauri::AppHandle;

use crate::{
    errors::{AppError, AppResult},
    services::app_config::load_app_config,
};

const YOUTUBE_API_BASE_URL: &str = "https://www.googleapis.com/youtube/v3";

#[derive(Debug, Clone)]
pub struct YoutubePlaylistBundle {
    pub playlist: YoutubePlaylistMetadata,
    pub videos: Vec<YoutubePlaylistVideo>,
}

#[derive(Debug, Clone)]
pub struct YoutubePlaylistMetadata {
    pub youtube_playlist_id: String,
    pub title: String,
    pub description: Option<String>,
    pub channel_title: Option<String>,
    pub thumbnail_url: Option<String>,
    pub video_count: i64,
}

#[derive(Debug, Clone)]
pub struct YoutubePlaylistVideo {
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

#[derive(Debug, Clone)]
pub struct YoutubeService {
    api_key: String,
    client: Client,
}

impl YoutubeService {
    pub fn new(app: &AppHandle) -> AppResult<Self> {
        let config = load_app_config(app)?;
        let api_key = config
            .config
            .youtube_api_key()
            .ok_or(AppError::MissingYoutubeApiKey)?
            .to_string();

        Ok(Self {
            api_key,
            client: Client::builder()
                .user_agent("playlist-browser/0.1.0")
                .build()
                .map_err(|_| AppError::YoutubeApiError)?,
        })
    }

    pub async fn fetch_playlist_bundle(&self, playlist_id: &str) -> AppResult<YoutubePlaylistBundle> {
        let playlist = self.fetch_playlist_metadata(playlist_id).await?;
        let playlist_items = self.fetch_playlist_items(playlist_id).await?;
        let videos_by_id = self.fetch_videos_by_ids(&playlist_items).await?;

        let mut videos = Vec::with_capacity(playlist_items.len());

        for (index, item) in playlist_items.iter().enumerate() {
            if let Some(mapped_video) = map_playlist_video(item, &videos_by_id, index as i64) {
                videos.push(mapped_video);
            }
        }

        Ok(YoutubePlaylistBundle { playlist, videos })
    }

    async fn fetch_playlist_metadata(
        &self,
        playlist_id: &str,
    ) -> AppResult<YoutubePlaylistMetadata> {
        let response: YoutubeListResponse<YoutubePlaylistResource> = self
            .request_json(
                "playlists",
                vec![
                    ("part", "snippet,contentDetails".to_string()),
                    ("id", playlist_id.to_string()),
                    ("maxResults", "1".to_string()),
                ],
            )
            .await?;

        let playlist = response
            .items
            .into_iter()
            .next()
            .ok_or(AppError::PlaylistNotFound)?;

        Ok(YoutubePlaylistMetadata {
            youtube_playlist_id: playlist.id,
            title: playlist.snippet.title,
            description: playlist.snippet.description,
            channel_title: playlist.snippet.channel_title,
            thumbnail_url: best_thumbnail_url(playlist.snippet.thumbnails.as_ref()),
            video_count: playlist
                .content_details
                .and_then(|details| details.item_count)
                .unwrap_or(0),
        })
    }

    async fn fetch_playlist_items(
        &self,
        playlist_id: &str,
    ) -> AppResult<Vec<YoutubePlaylistItemResource>> {
        let mut items = Vec::new();
        let mut page_token: Option<String> = None;

        loop {
            let mut params = vec![
                ("part", "snippet,contentDetails".to_string()),
                ("playlistId", playlist_id.to_string()),
                ("maxResults", "50".to_string()),
            ];

            if let Some(token) = &page_token {
                params.push(("pageToken", token.clone()));
            }

            let response: YoutubeListResponse<YoutubePlaylistItemResource> =
                self.request_json("playlistItems", params).await?;

            items.extend(response.items);

            if let Some(next_page_token) = response.next_page_token {
                page_token = Some(next_page_token);
            } else {
                break;
            }
        }

        Ok(items)
    }

    async fn fetch_videos_by_ids(
        &self,
        playlist_items: &[YoutubePlaylistItemResource],
    ) -> AppResult<HashMap<String, YoutubeVideoResource>> {
        let mut ordered_video_ids = Vec::new();
        let mut seen = HashSet::new();

        for item in playlist_items {
            let video_id = item
                .content_details
                .as_ref()
                .and_then(|details| details.video_id.clone())
                .or_else(|| {
                    item.snippet
                        .as_ref()
                        .and_then(|snippet| snippet.resource_id.as_ref())
                        .and_then(|resource| resource.video_id.clone())
                });

            if let Some(video_id) = video_id {
                if seen.insert(video_id.clone()) {
                    ordered_video_ids.push(video_id);
                }
            }
        }

        let mut videos_by_id = HashMap::new();

        for chunk in ordered_video_ids.chunks(50) {
            let response: YoutubeListResponse<YoutubeVideoResource> = self
                .request_json(
                    "videos",
                    vec![
                        ("part", "snippet,contentDetails".to_string()),
                        ("id", chunk.join(",")),
                        ("maxResults", "50".to_string()),
                    ],
                )
                .await?;

            for video in response.items {
                videos_by_id.insert(video.id.clone(), video);
            }
        }

        Ok(videos_by_id)
    }

    async fn request_json<T>(&self, endpoint: &str, params: Vec<(&str, String)>) -> AppResult<T>
    where
        T: DeserializeOwned,
    {
        let mut query = params;
        query.push(("key", self.api_key.clone()));

        let response = self
            .client
            .get(format!("{YOUTUBE_API_BASE_URL}/{endpoint}"))
            .query(&query)
            .send()
            .await
            .map_err(|_| AppError::YoutubeApiError)?;

        let status = response.status();

        if status.is_success() {
            return response.json::<T>().await.map_err(|_| AppError::YoutubeApiError);
        }

        let api_error = response.json::<YoutubeApiErrorResponse>().await.ok();
        Err(map_youtube_error(status, api_error.as_ref()))
    }
}

fn map_playlist_video(
    item: &YoutubePlaylistItemResource,
    videos_by_id: &HashMap<String, YoutubeVideoResource>,
    fallback_position: i64,
) -> Option<YoutubePlaylistVideo> {
    let youtube_video_id = item
        .content_details
        .as_ref()
        .and_then(|details| details.video_id.clone())
        .or_else(|| {
            item.snippet
                .as_ref()
                .and_then(|snippet| snippet.resource_id.as_ref())
                .and_then(|resource| resource.video_id.clone())
        })?;

    let playlist_snippet = item.snippet.as_ref();
    let video = videos_by_id.get(&youtube_video_id);
    let video_snippet = video.and_then(|details| details.snippet.as_ref());
    let video_content = video.and_then(|details| details.content_details.as_ref());

    Some(YoutubePlaylistVideo {
        youtube_video_id,
        youtube_playlist_item_id: Some(item.id.clone()),
        title: video_snippet
            .and_then(|snippet| snippet.title.clone())
            .or_else(|| playlist_snippet.map(|snippet| snippet.title.clone()))
            .unwrap_or_else(|| "Vidéo sans titre".to_string()),
        description: video_snippet
            .and_then(|snippet| snippet.description.clone())
            .or_else(|| playlist_snippet.and_then(|snippet| snippet.description.clone())),
        channel_title: video_snippet
            .and_then(|snippet| snippet.channel_title.clone())
            .or_else(|| playlist_snippet.and_then(|snippet| snippet.channel_title.clone()))
            .or_else(|| playlist_snippet.and_then(|snippet| snippet.video_owner_channel_title.clone())),
        thumbnail_url: video_snippet
            .and_then(|snippet| best_thumbnail_url(snippet.thumbnails.as_ref()))
            .or_else(|| playlist_snippet.and_then(|snippet| best_thumbnail_url(snippet.thumbnails.as_ref()))),
        published_at: video_snippet
            .and_then(|snippet| snippet.published_at.clone())
            .or_else(|| playlist_snippet.and_then(|snippet| snippet.published_at.clone())),
        duration_iso8601: video_content.and_then(|content| content.duration.clone()),
        duration_seconds: video_content
            .and_then(|content| content.duration.as_deref())
            .and_then(parse_duration_seconds),
        position: playlist_snippet
            .and_then(|snippet| snippet.position)
            .unwrap_or(fallback_position),
    })
}

fn best_thumbnail_url(thumbnails: Option<&YoutubeThumbnails>) -> Option<String> {
    thumbnails
        .and_then(|value| {
            value
                .maxres
                .as_ref()
                .or(value.standard.as_ref())
                .or(value.high.as_ref())
                .or(value.medium.as_ref())
                .or(value.default_thumbnail.as_ref())
        })
        .map(|thumbnail| thumbnail.url.clone())
}

pub fn parse_duration_seconds(duration: &str) -> Option<i64> {
    if !duration.starts_with('P') {
        return None;
    }

    let mut total = 0_i64;
    let mut number = String::new();
    let mut in_time = false;

    for character in duration.chars().skip(1) {
        match character {
            'T' => {
                in_time = true;
            }
            '0'..='9' => {
                number.push(character);
            }
            'H' if in_time => {
                total += number.parse::<i64>().ok()? * 3600;
                number.clear();
            }
            'M' if in_time => {
                total += number.parse::<i64>().ok()? * 60;
                number.clear();
            }
            'S' if in_time => {
                total += number.parse::<i64>().ok()?;
                number.clear();
            }
            _ => return None,
        }
    }

    if number.is_empty() {
        Some(total)
    } else {
        None
    }
}

fn map_youtube_error(status: StatusCode, error: Option<&YoutubeApiErrorResponse>) -> AppError {
    let reason = error
        .and_then(|payload| payload.error.errors.first())
        .and_then(|detail| detail.reason.as_deref());

    match reason {
        Some("playlistNotFound") => AppError::PlaylistNotFound,
        Some("playlistForbidden") | Some("forbidden") | Some("privatePlaylist") => {
            AppError::PlaylistInaccessible
        }
        Some("quotaExceeded")
        | Some("dailyLimitExceeded")
        | Some("dailyLimitExceededUnreg")
        | Some("rateLimitExceeded") => AppError::QuotaExceeded,
        _ => match status {
            StatusCode::NOT_FOUND => AppError::PlaylistNotFound,
            StatusCode::FORBIDDEN => AppError::PlaylistInaccessible,
            StatusCode::TOO_MANY_REQUESTS => AppError::QuotaExceeded,
            _ => AppError::YoutubeApiError,
        },
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct YoutubeListResponse<T> {
    items: Vec<T>,
    next_page_token: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct YoutubePlaylistResource {
    id: String,
    snippet: YoutubePlaylistSnippet,
    content_details: Option<YoutubePlaylistContentDetails>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct YoutubePlaylistSnippet {
    title: String,
    description: Option<String>,
    channel_title: Option<String>,
    thumbnails: Option<YoutubeThumbnails>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct YoutubePlaylistContentDetails {
    item_count: Option<i64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct YoutubePlaylistItemResource {
    id: String,
    snippet: Option<YoutubePlaylistItemSnippet>,
    content_details: Option<YoutubePlaylistItemContentDetails>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct YoutubePlaylistItemSnippet {
    title: String,
    description: Option<String>,
    channel_title: Option<String>,
    thumbnails: Option<YoutubeThumbnails>,
    position: Option<i64>,
    published_at: Option<String>,
    resource_id: Option<YoutubeResourceId>,
    video_owner_channel_title: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct YoutubePlaylistItemContentDetails {
    video_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct YoutubeResourceId {
    video_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct YoutubeVideoResource {
    id: String,
    snippet: Option<YoutubeVideoSnippet>,
    content_details: Option<YoutubeVideoContentDetails>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct YoutubeVideoSnippet {
    title: Option<String>,
    description: Option<String>,
    channel_title: Option<String>,
    thumbnails: Option<YoutubeThumbnails>,
    published_at: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct YoutubeVideoContentDetails {
    duration: Option<String>,
}

#[derive(Debug, Deserialize)]
struct YoutubeThumbnails {
    #[serde(rename = "default")]
    default_thumbnail: Option<YoutubeThumbnail>,
    medium: Option<YoutubeThumbnail>,
    high: Option<YoutubeThumbnail>,
    standard: Option<YoutubeThumbnail>,
    maxres: Option<YoutubeThumbnail>,
}

#[derive(Debug, Deserialize)]
struct YoutubeThumbnail {
    url: String,
}

#[derive(Debug, Deserialize)]
struct YoutubeApiErrorResponse {
    error: YoutubeApiErrorPayload,
}

#[derive(Debug, Deserialize)]
struct YoutubeApiErrorPayload {
    errors: Vec<YoutubeApiErrorDetail>,
}

#[derive(Debug, Deserialize)]
struct YoutubeApiErrorDetail {
    reason: Option<String>,
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{map_playlist_video, parse_duration_seconds, YoutubePlaylistItemResource, YoutubeVideoResource};

    #[test]
    fn parses_iso8601_duration_to_seconds() {
        assert_eq!(parse_duration_seconds("PT1H2M3S"), Some(3723));
        assert_eq!(parse_duration_seconds("PT4M"), Some(240));
        assert_eq!(parse_duration_seconds("PT0S"), Some(0));
        assert_eq!(parse_duration_seconds("P1DT2H"), None);
    }

    #[test]
    fn maps_playlist_item_with_video_metadata() {
        let playlist_item: YoutubePlaylistItemResource = serde_json::from_str(
            r#"{
              "id": "item-1",
              "snippet": {
                "title": "Titre fallback",
                "description": "Desc fallback",
                "channelTitle": "Chaîne fallback",
                "position": 2,
                "resourceId": { "videoId": "video-1" },
                "thumbnails": {
                  "medium": { "url": "https://fallback.example/medium.jpg" }
                }
              },
              "contentDetails": {
                "videoId": "video-1"
              }
            }"#,
        )
        .expect("playlist item should deserialize");

        let video_resource: YoutubeVideoResource = serde_json::from_str(
            r#"{
              "id": "video-1",
              "snippet": {
                "title": "Titre vidéo",
                "description": "Description vidéo",
                "channelTitle": "Chaîne vidéo",
                "publishedAt": "2026-04-05T10:00:00Z",
                "thumbnails": {
                  "high": { "url": "https://video.example/high.jpg" }
                }
              },
              "contentDetails": {
                "duration": "PT5M4S"
              }
            }"#,
        )
        .expect("video resource should deserialize");

        let mut videos_by_id = HashMap::new();
        videos_by_id.insert("video-1".to_string(), video_resource);

        let mapped = map_playlist_video(&playlist_item, &videos_by_id, 0)
            .expect("playlist video should map");

        assert_eq!(mapped.youtube_video_id, "video-1");
        assert_eq!(mapped.title, "Titre vidéo");
        assert_eq!(mapped.duration_seconds, Some(304));
        assert_eq!(mapped.thumbnail_url.as_deref(), Some("https://video.example/high.jpg"));
        assert_eq!(mapped.position, 2);
    }
}
