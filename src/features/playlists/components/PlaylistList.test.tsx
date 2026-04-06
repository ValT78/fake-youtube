import { render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { PlaylistList } from "./PlaylistList";
import type { PlaylistSummary } from "../../../lib/types";

const playlists: PlaylistSummary[] = [
  {
    id: "playlist:one",
    youtubePlaylistId: "PL123",
    title: "Playlist One",
    description: null,
    channelTitle: "Chaîne 1",
    thumbnailUrl: null,
    videoCount: 12,
    sourceUrl: "https://www.youtube.com/playlist?list=PL123",
    createdAt: "2026-04-05T12:00:00Z",
    updatedAt: "2026-04-05T12:00:00Z",
    lastSyncedAt: "2026-04-05T12:00:00Z",
  },
  {
    id: "playlist:two",
    youtubePlaylistId: "PL456",
    title: "Playlist Two",
    description: null,
    channelTitle: "Chaîne 2",
    thumbnailUrl: null,
    videoCount: 3,
    sourceUrl: "https://www.youtube.com/playlist?list=PL456",
    createdAt: "2026-04-05T12:00:00Z",
    updatedAt: "2026-04-05T12:00:00Z",
    lastSyncedAt: "2026-04-05T12:00:00Z",
  },
];

describe("PlaylistList", () => {
  it("affiche les playlists importées", () => {
    render(<PlaylistList playlists={playlists} onOpenPlaylist={vi.fn()} />);

    expect(screen.getByRole("button", { name: /Playlist One/i })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /Playlist Two/i })).toBeInTheDocument();
  });
});

