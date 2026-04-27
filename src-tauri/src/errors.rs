use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Clone, thiserror::Error)]
pub enum AppError {
    #[error("L'URL fournie ne correspond pas à une playlist YouTube valide.")]
    InvalidPlaylistUrl,
    #[error("L'URL fournie ne contient pas d'identifiant de playlist.")]
    MissingPlaylistId,
    #[error("La playlist demandée est introuvable.")]
    PlaylistNotFound,
    #[error("Cette playlist est privée ou inaccessible.")]
    PlaylistInaccessible,
    #[error("Le quota de l'API YouTube est dépassé ou l'accès a été refusé.")]
    QuotaExceeded,
    #[error("La clé YouTube API est requise pour importer une playlist. Renseigne-la dans le fichier JSON de configuration.")]
    MissingYoutubeApiKey,
    #[error("Une erreur est survenue pendant l'appel à l'API YouTube.")]
    YoutubeApiError,
    #[error("yt-dlp est introuvable. Installe yt-dlp ou définis la variable d'environnement YTDLP_PATH.")]
    YtDlpUnavailable,
    #[error("Impossible de résoudre un flux lisible avec yt-dlp. Vérifie que yt-dlp est à jour.")]
    YtDlpExtractionFailed,
    #[error("VLC est introuvable. Installe VLC ou définis la variable d'environnement VLC_PATH.")]
    VlcUnavailable,
    #[error("Impossible de lancer VLC avec cette vidéo.")]
    VlcLaunchFailed,
    #[error("Impossible de déterminer où stocker le fichier JSON de configuration.")]
    ConfigPathUnavailable,
    #[error("Impossible de lire le fichier JSON de configuration.")]
    ConfigReadFailed,
    #[error("Impossible d'enregistrer le fichier JSON de configuration.")]
    ConfigWriteFailed,
    #[error("La base locale SQLite est indisponible.")]
    DatabaseUnavailable,
    #[error("{0}")]
    Internal(String),
}

impl AppError {
    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal(message.into())
    }

    fn kind(&self) -> &'static str {
        match self {
            Self::InvalidPlaylistUrl => "invalid_playlist_url",
            Self::MissingPlaylistId => "missing_playlist_id",
            Self::PlaylistNotFound => "playlist_not_found",
            Self::PlaylistInaccessible => "playlist_inaccessible",
            Self::QuotaExceeded => "quota_exceeded",
            Self::MissingYoutubeApiKey => "missing_youtube_api_key",
            Self::YoutubeApiError => "youtube_api_error",
            Self::YtDlpUnavailable => "ytdlp_unavailable",
            Self::YtDlpExtractionFailed => "ytdlp_extraction_failed",
            Self::VlcUnavailable => "vlc_unavailable",
            Self::VlcLaunchFailed => "vlc_launch_failed",
            Self::ConfigPathUnavailable => "config_path_unavailable",
            Self::ConfigReadFailed => "config_read_failed",
            Self::ConfigWriteFailed => "config_write_failed",
            Self::DatabaseUnavailable => "database_unavailable",
            Self::Internal(_) => "internal",
        }
    }
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("AppError", 2)?;
        state.serialize_field("kind", self.kind())?;
        state.serialize_field("message", &self.to_string())?;
        state.end()
    }
}

impl From<reqwest::Error> for AppError {
    fn from(_: reqwest::Error) -> Self {
        Self::YoutubeApiError
    }
}

impl From<sqlx::Error> for AppError {
    fn from(_: sqlx::Error) -> Self {
        Self::DatabaseUnavailable
    }
}

impl From<std::io::Error> for AppError {
    fn from(_: std::io::Error) -> Self {
        Self::DatabaseUnavailable
    }
}

impl From<url::ParseError> for AppError {
    fn from(_: url::ParseError) -> Self {
        Self::InvalidPlaylistUrl
    }
}

#[cfg(test)]
mod tests {
    use super::AppError;

    #[test]
    fn serializes_unit_variants_with_kind_and_message() {
        let json = serde_json::to_value(AppError::MissingYoutubeApiKey)
            .expect("app error should serialize");

        assert_eq!(json["kind"], "missing_youtube_api_key");
        assert_eq!(
            json["message"],
            "La clé YouTube API est requise pour importer une playlist. Renseigne-la dans le fichier JSON de configuration."
        );
    }
}
