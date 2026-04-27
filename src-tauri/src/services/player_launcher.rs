use std::path::Path;
use std::process::Command;

use tauri::AppHandle;

use crate::errors::{AppError, AppResult};
use crate::services::app_config::load_app_config;

pub fn launch_video_in_vlc(app: &AppHandle, video_id: &str) -> AppResult<()> {
    let app_config = load_app_config(app)?.config;
    let vlc_binary = resolve_vlc_binary(app_config.vlc_path()).ok_or(AppError::VlcUnavailable)?;
    let ytdlp_binary =
        resolve_ytdlp_binary(app_config.ytdlp_path()).ok_or(AppError::YtDlpUnavailable)?;
    let video_url = build_watch_url(video_id);
    let stream_url = extract_stream_url(&ytdlp_binary, &video_url)?;

    Command::new(&vlc_binary)
        .arg(stream_url)
        .spawn()
        .map(|_| ())
        .map_err(|error| match error.kind() {
            std::io::ErrorKind::NotFound => AppError::VlcUnavailable,
            _ => AppError::VlcLaunchFailed,
        })
}

fn resolve_vlc_binary(custom_path: Option<&str>) -> Option<String> {
    resolve_binary(custom_path, vlc_candidates())
}

fn resolve_ytdlp_binary(custom_path: Option<&str>) -> Option<String> {
    resolve_binary(custom_path, ytdlp_candidates())
}

fn resolve_binary(custom_path: Option<&str>, candidates: &'static [&'static str]) -> Option<String> {
    if let Some(path) = custom_path {
        if !path.trim().is_empty() {
            return Some(path.to_string());
        }
    }

    for candidate in candidates {
        if looks_like_command(candidate) || Path::new(candidate).exists() {
            return Some(candidate.to_string());
        }
    }

    None
}

fn vlc_candidates() -> &'static [&'static str] {
    #[cfg(target_os = "windows")]
    {
        &[
            "vlc.exe",
            r"C:\Program Files\VideoLAN\VLC\vlc.exe",
            r"C:\Program Files (x86)\VideoLAN\VLC\vlc.exe",
        ]
    }

    #[cfg(not(target_os = "windows"))]
    {
        &["vlc", "/usr/bin/vlc", "/snap/bin/vlc"]
    }
}

fn ytdlp_candidates() -> &'static [&'static str] {
    #[cfg(target_os = "windows")]
    {
        &[
            "yt-dlp.exe",
            "yt-dlp",
            r"C:\Program Files\yt-dlp\yt-dlp.exe",
        ]
    }

    #[cfg(not(target_os = "windows"))]
    {
        &["yt-dlp", "/usr/bin/yt-dlp", "/usr/local/bin/yt-dlp"]
    }
}

fn looks_like_command(candidate: &str) -> bool {
    !candidate.contains(std::path::MAIN_SEPARATOR)
}

fn build_watch_url(video_id: &str) -> String {
    format!("https://www.youtube.com/watch?v={video_id}")
}

fn extract_stream_url(ytdlp_binary: &str, video_url: &str) -> AppResult<String> {
    let output = Command::new(ytdlp_binary)
        .args([
            "--no-playlist",
            "--no-warnings",
            "--get-url",
            "--format",
            "best[acodec!=none][vcodec!=none]/best",
            video_url,
        ])
        .output()
        .map_err(|error| match error.kind() {
            std::io::ErrorKind::NotFound => AppError::YtDlpUnavailable,
            _ => AppError::YtDlpExtractionFailed,
        })?;

    if !output.status.success() {
        return Err(AppError::YtDlpExtractionFailed);
    }

    parse_first_stream_url(&output.stdout).ok_or(AppError::YtDlpExtractionFailed)
}

fn parse_first_stream_url(stdout: &[u8]) -> Option<String> {
    let text = std::str::from_utf8(stdout).ok()?;

    text.lines()
        .map(str::trim)
        .find(|line| !line.is_empty() && line.starts_with("http"))
        .map(ToOwned::to_owned)
}

#[cfg(test)]
mod tests {
    use super::{build_watch_url, parse_first_stream_url};

    #[test]
    fn builds_watch_url_for_vlc() {
        assert_eq!(
            build_watch_url("abc123"),
            "https://www.youtube.com/watch?v=abc123"
        );
    }

    #[test]
    fn extracts_first_stream_url_from_ytdlp_output() {
        let parsed = parse_first_stream_url(
            b"https://redirector.googlevideo.com/videoplayback?id=video123\n\n",
        );

        assert_eq!(
            parsed.as_deref(),
            Some("https://redirector.googlevideo.com/videoplayback?id=video123")
        );
    }

    #[test]
    fn ignores_non_http_lines_when_parsing_ytdlp_output() {
        let parsed = parse_first_stream_url(
            b"[youtube] Extracting URL\nhttps://redirector.googlevideo.com/videoplayback?id=video123\n",
        );

        assert_eq!(
            parsed.as_deref(),
            Some("https://redirector.googlevideo.com/videoplayback?id=video123")
        );
    }
}
