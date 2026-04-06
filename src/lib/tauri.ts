import type {
  DatabaseStatus,
  ImportPlaylistResult,
  ParsedPlaylistUrl,
  PlaylistDetail,
  PlaylistSummary,
} from "./types";
import { invokeTauri } from "./tauriRuntime";

export async function parsePlaylistUrl(
  sourceUrl: string,
): Promise<ParsedPlaylistUrl> {
  return invokeTauri("parse_playlist_url", { sourceUrl });
}

export async function importPlaylist(
  sourceUrl: string,
): Promise<ImportPlaylistResult> {
  return invokeTauri("import_playlist", { sourceUrl });
}

export async function listPlaylists(): Promise<PlaylistSummary[]> {
  return invokeTauri("list_playlists");
}

export async function getPlaylistDetail(
  playlistId: string,
): Promise<PlaylistDetail> {
  return invokeTauri("get_playlist_detail", { playlistId });
}

export async function getDatabaseStatus(): Promise<DatabaseStatus> {
  return invokeTauri("database_status");
}

export async function openVideoInVlc(videoId: string): Promise<void> {
  return invokeTauri("open_video_in_vlc", { videoId });
}
