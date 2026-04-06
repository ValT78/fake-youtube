import type { PlaylistVideoItem } from "../../../lib/types";

interface VideoQueueProps {
  videos: PlaylistVideoItem[];
  currentIndex: number;
  onSelect: (index: number) => void;
}

function formatDuration(durationSeconds: number | null): string {
  if (durationSeconds === null) {
    return "Durée inconnue";
  }

  const minutes = Math.floor(durationSeconds / 60);
  const seconds = durationSeconds % 60;

  return `${minutes}:${seconds.toString().padStart(2, "0")}`;
}

export function VideoQueue({
  videos,
  currentIndex,
  onSelect,
}: VideoQueueProps) {
  return (
    <aside className="queue-panel panel stack gap-sm">
      <div className="stack gap-xxs">
        <p className="eyebrow">Vidéos</p>
        <h2>File de lecture</h2>
      </div>

      <div className="queue-list" role="list" aria-label="Vidéos de la playlist">
        {videos.map((video, index) => (
          <button
            key={`${video.id}-${video.position}`}
            className={`queue-item${index === currentIndex ? " active" : ""}`}
            type="button"
            onClick={() => onSelect(index)}
          >
            {video.thumbnailUrl ? (
              <img className="queue-item-thumbnail" src={video.thumbnailUrl} alt="" />
            ) : (
              <div className="queue-item-thumbnail placeholder">Sans image</div>
            )}

            <div className="stack gap-xxs queue-item-copy">
              <span className="queue-position">#{video.position + 1}</span>
              <strong>{video.title}</strong>
              <span className="muted">
                {video.channelTitle ?? "Chaîne inconnue"} ·{" "}
                {formatDuration(video.durationSeconds)}
              </span>
            </div>
          </button>
        ))}
      </div>
    </aside>
  );
}
