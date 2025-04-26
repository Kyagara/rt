import type { Database, Statement } from 'better-sqlite3'

import { usersDB, feedsDB, emotesDB } from './database/dbs.js'
import { Platform } from '../shared/enums.js'
import { fetchTwitchUser } from './twitch/user.js'
import { updateUserEmotes } from './twitch/emote.js'
import { fetchChannelByID, fetchChannelByName } from './youtube/channel.js'

export async function getUser(
	platform: Platform,
	username?: string,
	id?: string
): Promise<User | null> {
	const table = platform === Platform.YouTube ? 'youtube' : 'twitch'

	let stmt: Statement
	if (username) {
		stmt = usersDB.prepare(`SELECT id, username, avatar FROM ${table} WHERE username = ?`)
	} else if (id) {
		stmt = usersDB.prepare(`SELECT id, username, avatar FROM ${table} WHERE id = ?`)
	} else {
		throw new Error('Invalid parameters')
	}

	const user = stmt.get(username || id) as User
	return user
}

export async function listUsers(platform?: Platform): Promise<User[]> {
	const users: User[] = []

	function fetchPlatformUsers(platform: Platform): User[] {
		const table = platform === Platform.YouTube ? 'youtube' : 'twitch'
		const stmt = usersDB.prepare(`SELECT id, username, display_name, avatar FROM ${table}`)
		const rows = stmt.all() as User[]

		return rows.map((row) => ({
			id: row.id,
			username: row.username,
			display_name: row.display_name,
			avatar: row.avatar,
			platform: platform
		}))
	}

	if (platform) {
		return fetchPlatformUsers(platform)
	}

	users.push(...fetchPlatformUsers(Platform.YouTube))
	users.push(...fetchPlatformUsers(Platform.Twitch))

	return users
}

export async function addUser(
	platform: Platform,
	username?: string,
	id?: string
): Promise<User | null> {
	let newUser: User | null = null

	if (platform === Platform.Twitch) {
		if (!username) throw new Error('Username not provided')

		const { user, emotes } = await fetchTwitchUser(username)
		newUser = user

		updateUserEmotes(username, emotes)

		const stmt = usersDB.prepare(
			'INSERT INTO twitch (id, username, display_name, avatar) VALUES (?, ?, ?, ?) ON CONFLICT (id) DO UPDATE SET avatar = ?, display_name = ?'
		)

		stmt.run(
			newUser.id,
			newUser.username,
			newUser.display_name,
			newUser.avatar,
			newUser.avatar,
			newUser.display_name
		)
	}

	if (platform === Platform.YouTube) {
		if (!username && !id) throw new Error('Username or ID not provided')

		if (id) {
			newUser = await fetchChannelByID(id)
		} else if (username) {
			newUser = await fetchChannelByName(username)
		}

		if (!newUser) throw new Error('User not found')

		const stmt = usersDB.prepare(
			'INSERT INTO youtube (id, username, display_name, avatar) VALUES (?, ?, ?, ?) ON CONFLICT (id) DO UPDATE SET avatar = ?, display_name = ?'
		)

		stmt.run(
			newUser.id,
			newUser.username,
			newUser.display_name,
			newUser.avatar,
			newUser.avatar,
			newUser.display_name
		)
	}

	return newUser
}

export async function removeUser(platform: Platform, username: string): Promise<string> {
	if (!username) throw new Error('Username not provided')

	const table = platform === Platform.YouTube ? 'youtube' : 'twitch'

	const deleteFrom = (db: Database, table: string): void => {
		db.prepare(`DELETE FROM ${table} WHERE username = ?`).run(username)
	}

	deleteFrom(usersDB, table)
	deleteFrom(feedsDB, table)
	if (platform === Platform.Twitch) {
		deleteFrom(emotesDB, table)
	}

	return username
}
