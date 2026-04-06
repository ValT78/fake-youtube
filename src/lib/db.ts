import Database from "@tauri-apps/plugin-sql";
import { ensureTauriRuntime } from "./tauriRuntime";

export const DATABASE_URL = "sqlite:playlist-browser.db";

let databasePromise: Promise<Database> | null = null;

export async function ensureLocalDatabase(): Promise<void> {
  ensureTauriRuntime();

  if (databasePromise === null) {
    databasePromise = Database.load(DATABASE_URL);
  }

  await databasePromise;
}
