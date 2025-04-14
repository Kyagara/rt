import { Database } from 'better-sqlite3'

export default [
	{
		version: 1,
		description: 'Create users table',
		up: (db: Database): void => {
			db.exec(`
        CREATE TABLE IF NOT EXISTS twitch (
          id TEXT PRIMARY KEY,
          username TEXT NOT NULL,
          display_name TEXT,
          avatar BLOB
        );
  
        CREATE TABLE IF NOT EXISTS youtube (
          id TEXT PRIMARY KEY,
          username TEXT NOT NULL,
          display_name TEXT,
          avatar BLOB
        );
      `)
		}
	}
]
