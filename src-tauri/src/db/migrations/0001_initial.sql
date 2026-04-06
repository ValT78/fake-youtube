CREATE TABLE IF NOT EXISTS playlists (
  id TEXT PRIMARY KEY,
  youtube_playlist_id TEXT NOT NULL UNIQUE,
  title TEXT NOT NULL,
  description TEXT,
  channel_title TEXT,
  thumbnail_url TEXT,
  video_count INTEGER NOT NULL DEFAULT 0,
  source_url TEXT NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  last_synced_at TEXT
);

CREATE TABLE IF NOT EXISTS videos (
  id TEXT PRIMARY KEY,
  youtube_video_id TEXT NOT NULL UNIQUE,
  title TEXT NOT NULL,
  description TEXT,
  channel_title TEXT,
  thumbnail_url TEXT,
  published_at TEXT,
  duration_iso8601 TEXT,
  duration_seconds INTEGER,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS playlist_items (
  id TEXT PRIMARY KEY,
  playlist_id TEXT NOT NULL,
  video_id TEXT NOT NULL,
  youtube_playlist_item_id TEXT,
  position INTEGER NOT NULL,
  created_at TEXT NOT NULL,
  UNIQUE(playlist_id, position),
  UNIQUE(playlist_id, video_id),
  FOREIGN KEY (playlist_id) REFERENCES playlists(id) ON DELETE CASCADE,
  FOREIGN KEY (video_id) REFERENCES videos(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_playlists_youtube_playlist_id
  ON playlists (youtube_playlist_id);

CREATE INDEX IF NOT EXISTS idx_videos_youtube_video_id
  ON videos (youtube_video_id);

CREATE INDEX IF NOT EXISTS idx_playlist_items_playlist_position
  ON playlist_items (playlist_id, position);

