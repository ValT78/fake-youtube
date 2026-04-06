use std::fs;
use std::path::PathBuf;

use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};
use sqlx::{query, query_as, SqlitePool};
use tauri::{AppHandle, Manager};
use tauri_plugin_sql::{Migration, MigrationKind};

use crate::errors::{AppError, AppResult};
use crate::models::{PlaylistDetail, PlaylistSummary, PlaylistVideoItem};
use crate::services::sync::PlaylistImportPayload;

pub const DB_PLUGIN_URL: &str = "sqlite:playlist-browser.db";
const DB_FILE_NAME: &str = "playlist-browser.db";

pub fn migrations() -> Vec<Migration> {
    vec![Migration {
        version: 1,
        description: "create_initial_tables",
        sql: include_str!("migrations/0001_initial.sql"),
        kind: MigrationKind::Up,
    }]
}

pub fn database_path(app: &AppHandle) -> AppResult<PathBuf> {
    let config_dir = app
        .path()
        .app_config_dir()
        .map_err(|_| AppError::DatabaseUnavailable)?;

    fs::create_dir_all(&config_dir)?;

    Ok(config_dir.join(DB_FILE_NAME))
}

pub async fn connect(app: &AppHandle) -> AppResult<SqlitePool> {
    let path = database_path(app)?;

    let options = SqliteConnectOptions::new()
        .filename(path)
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .foreign_keys(true);

    SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await
        .map_err(|_| AppError::DatabaseUnavailable)
}

pub async fn list_playlists(app: &AppHandle) -> AppResult<Vec<PlaylistSummary>> {
    let pool = connect(app).await?;

    query_as::<_, PlaylistSummary>(
        r#"
        SELECT
          id,
          youtube_playlist_id,
          title,
          description,
          channel_title,
          thumbnail_url,
          video_count,
          source_url,
          created_at,
          updated_at,
          last_synced_at
        FROM playlists
        ORDER BY COALESCE(last_synced_at, updated_at) DESC, title ASC
        "#,
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| AppError::DatabaseUnavailable)
}

pub async fn get_playlist_detail(app: &AppHandle, playlist_id: &str) -> AppResult<PlaylistDetail> {
    let pool = connect(app).await?;

    let playlist = query_as::<_, PlaylistSummary>(
        r#"
        SELECT
          id,
          youtube_playlist_id,
          title,
          description,
          channel_title,
          thumbnail_url,
          video_count,
          source_url,
          created_at,
          updated_at,
          last_synced_at
        FROM playlists
        WHERE id = ?
        "#,
    )
    .bind(playlist_id)
    .fetch_optional(&pool)
    .await
    .map_err(|_| AppError::DatabaseUnavailable)?
    .ok_or(AppError::PlaylistNotFound)?;

    let videos = query_as::<_, PlaylistVideoItem>(
        r#"
        SELECT
          videos.id,
          videos.youtube_video_id,
          playlist_items.youtube_playlist_item_id,
          videos.title,
          videos.description,
          videos.channel_title,
          videos.thumbnail_url,
          videos.published_at,
          videos.duration_iso8601,
          videos.duration_seconds,
          playlist_items.position
        FROM playlist_items
        INNER JOIN videos ON videos.id = playlist_items.video_id
        WHERE playlist_items.playlist_id = ?
        ORDER BY playlist_items.position ASC
        "#,
    )
    .bind(playlist_id)
    .fetch_all(&pool)
    .await
    .map_err(|_| AppError::DatabaseUnavailable)?;

    Ok(PlaylistDetail { playlist, videos })
}

pub async fn persist_playlist_import(
    app: &AppHandle,
    payload: &PlaylistImportPayload,
) -> AppResult<()> {
    let pool = connect(app).await?;
    let mut transaction = pool
        .begin()
        .await
        .map_err(|_| AppError::DatabaseUnavailable)?;

    query(
        r#"
        INSERT INTO playlists (
          id,
          youtube_playlist_id,
          title,
          description,
          channel_title,
          thumbnail_url,
          video_count,
          source_url,
          created_at,
          updated_at,
          last_synced_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ON CONFLICT(id) DO UPDATE SET
          youtube_playlist_id = excluded.youtube_playlist_id,
          title = excluded.title,
          description = excluded.description,
          channel_title = excluded.channel_title,
          thumbnail_url = excluded.thumbnail_url,
          video_count = excluded.video_count,
          source_url = excluded.source_url,
          updated_at = excluded.updated_at,
          last_synced_at = excluded.last_synced_at
        "#,
    )
    .bind(&payload.playlist.id)
    .bind(&payload.playlist.youtube_playlist_id)
    .bind(&payload.playlist.title)
    .bind(payload.playlist.description.as_deref())
    .bind(payload.playlist.channel_title.as_deref())
    .bind(payload.playlist.thumbnail_url.as_deref())
    .bind(payload.playlist.video_count)
    .bind(&payload.playlist.source_url)
    .bind(&payload.playlist.created_at)
    .bind(&payload.playlist.updated_at)
    .bind(payload.playlist.last_synced_at.as_deref())
    .execute(&mut *transaction)
    .await
    .map_err(|_| AppError::DatabaseUnavailable)?;

    for video in &payload.videos {
        query(
            r#"
            INSERT INTO videos (
              id,
              youtube_video_id,
              title,
              description,
              channel_title,
              thumbnail_url,
              published_at,
              duration_iso8601,
              duration_seconds,
              created_at,
              updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET
              youtube_video_id = excluded.youtube_video_id,
              title = excluded.title,
              description = excluded.description,
              channel_title = excluded.channel_title,
              thumbnail_url = excluded.thumbnail_url,
              published_at = excluded.published_at,
              duration_iso8601 = excluded.duration_iso8601,
              duration_seconds = excluded.duration_seconds,
              updated_at = excluded.updated_at
            "#,
        )
        .bind(&video.id)
        .bind(&video.youtube_video_id)
        .bind(&video.title)
        .bind(video.description.as_deref())
        .bind(video.channel_title.as_deref())
        .bind(video.thumbnail_url.as_deref())
        .bind(video.published_at.as_deref())
        .bind(video.duration_iso8601.as_deref())
        .bind(video.duration_seconds)
        .bind(&video.created_at)
        .bind(&video.updated_at)
        .execute(&mut *transaction)
        .await
        .map_err(|_| AppError::DatabaseUnavailable)?;
    }

    query("DELETE FROM playlist_items WHERE playlist_id = ?")
        .bind(&payload.playlist.id)
        .execute(&mut *transaction)
        .await
        .map_err(|_| AppError::DatabaseUnavailable)?;

    for item in &payload.playlist_items {
        query(
            r#"
            INSERT INTO playlist_items (
              id,
              playlist_id,
              video_id,
              youtube_playlist_item_id,
              position,
              created_at
            ) VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&item.id)
        .bind(&item.playlist_id)
        .bind(&item.video_id)
        .bind(item.youtube_playlist_item_id.as_deref())
        .bind(item.position)
        .bind(&item.created_at)
        .execute(&mut *transaction)
        .await
        .map_err(|_| AppError::DatabaseUnavailable)?;
    }

    transaction
        .commit()
        .await
        .map_err(|_| AppError::DatabaseUnavailable)
}
