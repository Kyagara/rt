import { app } from 'electron'
import path from 'path'
import SQLite, { type Database } from 'better-sqlite3'

const dbPath = (name: string): string => path.join(app.getPath('userData'), `${name}.db`)

let usersDB: Database
let feedsDB: Database
let emotesDB: Database

export const initDatabases = (): void => {
	usersDB = new SQLite(dbPath('users'))
	feedsDB = new SQLite(dbPath('feeds'))
	emotesDB = new SQLite(dbPath('emotes'))

	for (const db of [usersDB, feedsDB, emotesDB]) {
		db.pragma('journal_mode = WAL')
		db.pragma('synchronous = NORMAL')
	}
}

export { usersDB, feedsDB, emotesDB }
