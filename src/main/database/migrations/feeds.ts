import type { Database } from 'better-sqlite3'

export default [
	{
		version: 1,
		description: 'Create feeds table',
		up: (db: Database): void => {
			db.exec(`
        CREATE TABLE IF NOT EXISTS twitch (
          username TEXT PRIMARY KEY,
          started_at TEXT
        );

        CREATE TABLE IF NOT EXISTS youtube (
          id TEXT PRIMARY KEY,
          username TEXT NOT NULL,
          title TEXT,
          published_at INTEGER,
          view_count TEXT
        );

        CREATE INDEX idx_title ON youtube (title);
        CREATE INDEX idx_published_at ON youtube (published_at);
      `)
		}
	}
]
