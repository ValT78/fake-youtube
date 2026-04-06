import { FormEvent, useState } from "react";

interface AddPlaylistFormProps {
  busy: boolean;
  onSubmit: (url: string) => Promise<void>;
}

export function AddPlaylistForm({
  busy,
  onSubmit,
}: AddPlaylistFormProps) {
  const [url, setUrl] = useState("");

  async function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const trimmedUrl = url.trim();

    if (!trimmedUrl) {
      return;
    }

    await onSubmit(trimmedUrl);
    setUrl("");
  }

  return (
    <form className="panel stack gap-sm" onSubmit={handleSubmit}>
      <div className="stack gap-xs">
        <label className="label" htmlFor="playlist-url">
          Ajouter une playlist
        </label>
        <p className="muted">
          Collez une URL YouTube contenant un identifiant de playlist.
        </p>
      </div>

      <div className="form-row">
        <input
          id="playlist-url"
          className="input"
          type="url"
          value={url}
          onChange={(event) => setUrl(event.target.value)}
          placeholder="https://www.youtube.com/playlist?list=..."
          autoComplete="off"
          spellCheck={false}
          disabled={busy}
          required
        />
        <button className="button button-primary" type="submit" disabled={busy}>
          {busy ? "Import en cours..." : "Importer"}
        </button>
      </div>
    </form>
  );
}
