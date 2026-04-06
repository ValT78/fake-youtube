export interface AppErrorPayload {
  kind: string;
  message: string;
}

export interface DatabaseStatus {
  ready: boolean;
  databasePath: string;
}

export interface ParsedPlaylistUrl {
  originalUrl: string;
  canonicalUrl: string;
  playlistId: string;
}

export interface PlaylistSummary {
  id: string;
  youtubePlaylistId: string;
  title: string;
  description: string | null;
  channelTitle: string | null;
  thumbnailUrl: string | null;
  videoCount: number;
  sourceUrl: string;
  createdAt: string;
  updatedAt: string;
  lastSyncedAt: string | null;
}

export interface PlaylistVideoItem {
  id: string;
  youtubeVideoId: string;
  youtubePlaylistItemId: string | null;
  title: string;
  description: string | null;
  channelTitle: string | null;
  thumbnailUrl: string | null;
  publishedAt: string | null;
  durationIso8601: string | null;
  durationSeconds: number | null;
  position: number;
}

export interface PlaylistDetail extends PlaylistSummary {
  videos: PlaylistVideoItem[];
}

export interface ImportPlaylistResult {
  playlist: PlaylistSummary;
  importedVideoCount: number;
  source: "api";
}

