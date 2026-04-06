use url::Url;

use crate::{
    errors::{AppError, AppResult},
    models::ParsedPlaylistUrl,
};

const CANONICAL_PLAYLIST_PREFIX: &str = "https://www.youtube.com/playlist?list=";
const SUPPORTED_HOSTS: &[&str] = &[
    "youtube.com",
    "www.youtube.com",
    "m.youtube.com",
    "music.youtube.com",
    "youtu.be",
    "www.youtu.be",
];

pub fn parse_playlist_reference(input: &str) -> AppResult<ParsedPlaylistUrl> {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return Err(AppError::InvalidPlaylistUrl);
    }

    if looks_like_playlist_id(trimmed) {
        return Ok(ParsedPlaylistUrl {
            original_url: trimmed.to_string(),
            canonical_url: format!("{CANONICAL_PLAYLIST_PREFIX}{trimmed}"),
            playlist_id: trimmed.to_string(),
        });
    }

    let normalized = normalize_url(trimmed);
    let parsed = Url::parse(&normalized)?;
    let host = parsed
        .host_str()
        .ok_or(AppError::InvalidPlaylistUrl)?
        .to_ascii_lowercase();

    if !SUPPORTED_HOSTS.contains(&host.as_str()) {
        return Err(AppError::InvalidPlaylistUrl);
    }

    let playlist_id = parsed
        .query_pairs()
        .find_map(|(key, value)| (key == "list").then(|| value.into_owned()))
        .filter(|value| looks_like_playlist_id(value))
        .ok_or(AppError::MissingPlaylistId)?;

    Ok(ParsedPlaylistUrl {
        original_url: trimmed.to_string(),
        canonical_url: format!("{CANONICAL_PLAYLIST_PREFIX}{playlist_id}"),
        playlist_id,
    })
}

fn normalize_url(input: &str) -> String {
    if input.starts_with("https://") || input.starts_with("http://") {
        input.to_string()
    } else {
        format!("https://{input}")
    }
}

fn looks_like_playlist_id(value: &str) -> bool {
    value.len() >= 10
        && value
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || character == '-' || character == '_')
}

#[cfg(test)]
mod tests {
    use super::parse_playlist_reference;
    use crate::errors::AppError;

    #[test]
    fn parses_playlist_page_url() {
        let parsed =
            parse_playlist_reference("https://www.youtube.com/playlist?list=PL1234567890")
                .expect("playlist url should parse");

        assert_eq!(parsed.playlist_id, "PL1234567890");
    }

    #[test]
    fn parses_watch_url_with_list_parameter() {
        let parsed = parse_playlist_reference(
            "https://www.youtube.com/watch?v=abc123&list=PLabcdefghijkl",
        )
        .expect("watch url should parse");

        assert_eq!(parsed.playlist_id, "PLabcdefghijkl");
    }

    #[test]
    fn parses_short_url_with_list_parameter() {
        let parsed = parse_playlist_reference(
            "https://youtu.be/abc123?t=42&list=PLshortlink42",
        )
        .expect("short url should parse");

        assert_eq!(parsed.playlist_id, "PLshortlink42");
    }

    #[test]
    fn parses_raw_playlist_id() {
        let parsed =
            parse_playlist_reference("PLrawplaylist42").expect("raw playlist id should parse");

        assert_eq!(parsed.playlist_id, "PLrawplaylist42");
    }

    #[test]
    fn rejects_supported_url_without_list_parameter() {
        let error =
            parse_playlist_reference("https://www.youtube.com/watch?v=abc123").unwrap_err();

        assert!(matches!(error, AppError::MissingPlaylistId));
    }

    #[test]
    fn rejects_unknown_host() {
        let error =
            parse_playlist_reference("https://example.com/playlist?list=PL1234567890")
                .unwrap_err();

        assert!(matches!(error, AppError::InvalidPlaylistUrl));
    }
}

