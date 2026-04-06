import { fireEvent, render, screen } from "@testing-library/react";
import { useState } from "react";
import { describe, expect, it } from "vitest";
import { PlaylistPlayer } from "./PlaylistPlayer";
import {
  getNextVideoIndex,
  getPreviousVideoIndex,
} from "../playerState";
import type { PlaylistVideoItem } from "../../../lib/types";

const videos: PlaylistVideoItem[] = [
  {
    id: "video:a",
    youtubeVideoId: "aaa111",
    youtubePlaylistItemId: "item:1",
    title: "Vidéo A",
    description: null,
    channelTitle: "Chaîne",
    thumbnailUrl: null,
    publishedAt: null,
    durationIso8601: "PT3M10S",
    durationSeconds: 190,
    position: 0,
  },
  {
    id: "video:b",
    youtubeVideoId: "bbb222",
    youtubePlaylistItemId: "item:2",
    title: "Vidéo B",
    description: null,
    channelTitle: "Chaîne",
    thumbnailUrl: null,
    publishedAt: null,
    durationIso8601: "PT4M",
    durationSeconds: 240,
    position: 1,
  },
];

function PlaylistPlayerHarness() {
  const [index, setIndex] = useState(0);

  return (
    <PlaylistPlayer
      playlistTitle="Test"
      currentVideo={videos[index] ?? null}
      canGoPrevious={index > 0}
      canGoNext={index < videos.length - 1}
      launchBusy={false}
      onLaunch={() => {}}
      onPrevious={() => setIndex(getPreviousVideoIndex(index))}
      onNext={() => setIndex(getNextVideoIndex(index, videos.length))}
    />
  );
}

describe("PlaylistPlayer", () => {
  it("navigue vers la vidéo suivante puis précédente", () => {
    render(<PlaylistPlayerHarness />);

    expect(screen.getByRole("heading", { name: "Vidéo A" })).toBeInTheDocument();

    fireEvent.click(screen.getByRole("button", { name: "Suivante" }));
    expect(screen.getByRole("heading", { name: "Vidéo B" })).toBeInTheDocument();

    fireEvent.click(screen.getByRole("button", { name: "Précédente" }));
    expect(screen.getByRole("heading", { name: "Vidéo A" })).toBeInTheDocument();
  });
});
