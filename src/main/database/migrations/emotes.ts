import type { Database } from 'better-sqlite3'

export default [
	{
		version: 1,
		description: 'Create emotes table',
		up: (db: Database): void => {
			db.exec(`
        CREATE TABLE IF NOT EXISTS twitch (
          username TEXT NOT NULL,
          name TEXT NOT NULL,
          url TEXT,
          width INTEGER,
          height INTEGER,
          PRIMARY KEY (username, name)
        );
      `)
		}
	}
]
