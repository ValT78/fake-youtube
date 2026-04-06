use std::env;
use std::path::Path;
use std::process::Command;

use crate::errors::{AppError, AppResult};

const VLC_PATH_ENV: &str = "VLC_PATH";

pub fn launch_video_in_vlc(video_id: &str) -> AppResult<()> {
    let vlc_binary = resolve_vlc_binary().ok_or(AppError::VlcUnavailable)?;
    let video_url = build_watch_url(video_id);

    Command::new(&vlc_binary)
        .arg(video_url)
        .spawn()
        .map(|_| ())
        .map_err(|error| match error.kind() {
            std::io::ErrorKind::NotFound => AppError::VlcUnavailable,
            _ => AppError::VlcLaunchFailed,
        })
}

fn resolve_vlc_binary() -> Option<String> {
    if let Ok(custom_path) = env::var(VLC_PATH_ENV) {
        let trimmed_path = custom_path.trim();

        if !trimmed_path.is_empty() {
            return Some(trimmed_path.to_string());
        }
    }

    for candidate in candidate_binaries() {
        if looks_like_command(candidate) || Path::new(candidate).exists() {
            return Some(candidate.to_string());
        }
    }

    None
}

fn candidate_binaries() -> &'static [&'static str] {
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

fn looks_like_command(candidate: &str) -> bool {
    !candidate.contains(std::path::MAIN_SEPARATOR)
}

fn build_watch_url(video_id: &str) -> String {
    format!("https://www.youtube.com/watch?v={video_id}")
}

#[cfg(test)]
mod tests {
    use super::build_watch_url;

    #[test]
    fn builds_watch_url_for_vlc() {
        assert_eq!(
            build_watch_url("abc123"),
            "https://www.youtube.com/watch?v=abc123"
        );
    }
}
