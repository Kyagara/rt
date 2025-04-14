import type { Database } from 'better-sqlite3'

import { usersDB, feedsDB, emotesDB } from './dbs'
import usersMigrations from './migrations/users'
import feedsMigrations from './migrations/feeds'
import emotesMigrations from './migrations/emotes'

export interface Migration {
	version: number
	description: string
	up: (db: Database) => void
}

export function migrate(dbName: string, db: Database, migrations: Migration[]): void {
	const currentVersion = db.pragma('user_version', { simple: true }) as number

	const maxVersion = Math.max(...migrations.map((m) => m.version))

	if (currentVersion === maxVersion) {
		return
	}

	const toApply = migrations
		.filter((m) => m.version > currentVersion && m.version <= maxVersion)
		.sort((a, b) => a.version - b.version)

	const upgrade = db.transaction(() => {
		for (const migration of toApply) {
			console.log(`[${dbName}] Applying: ${migration.description} (v${migration.version})`)

			if (typeof migration.up === 'string') {
				db.exec(migration.up)
			} else {
				migration.up(db)
			}

			db.pragma(`user_version = ${migration.version}`)
		}
	})

	upgrade()
	console.log(`[${dbName}] Upgraded to v${maxVersion}`)
}

export function runMigrations(): void {
	migrate('users', usersDB, usersMigrations)
	migrate('feeds', feedsDB, feedsMigrations)
	migrate('emotes', emotesDB, emotesMigrations)
}
