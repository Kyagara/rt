import { Platform } from '$shared/enums.js'
import { usersDB, feedsDB } from './database/dbs.js'
import { fetchLiveNow } from './twitch/stream.js'
import { fetchFeedVideos } from './youtube/video.js'

export function getFeed(platform: Platform, lastPublishedAt?: number): Feed {
	if (platform === Platform.Twitch) {
		try {
			const stmt = feedsDB.prepare('SELECT username, started_at FROM twitch')
			const rows = stmt.all() as LiveNow[]

			const feed: LiveNow[] = rows.map((row) => ({
				username: row.username,
				started_at: row.started_at
			}))

			return {
				twitch: feed,
				youtube: null
			}
		} catch (err) {
			throw new Error(`Querying feed: ${err}`)
		}
	}

	if (platform === Platform.YouTube) {
		let sql: string
		if (lastPublishedAt) {
			sql =
				'SELECT id, username, title, published_at, view_count FROM youtube WHERE published_at < ? ORDER BY published_at DESC LIMIT 50'
		} else {
			sql =
				'SELECT id, username, title, published_at, view_count FROM youtube ORDER BY published_at DESC LIMIT 50'
		}

		const stmt = feedsDB.prepare(sql)
		const rows = lastPublishedAt
			? (stmt.all(lastPublishedAt) as FeedVideo[])
			: (stmt.all() as FeedVideo[])

		const feed: FeedVideo[] = rows.map((row) => ({
			id: row.id,
			username: row.username,
			title: row.title,
			published_at: row.published_at,
			view_count: row.view_count
		}))

		return {
			twitch: null,
			youtube: feed
		}
	}

	throw new Error(`Invalid platform '${platform}'`)
}

export async function refreshFeed(platform: Platform): Promise<Feed> {
	if (platform === Platform.Twitch) {
		try {
			const getStmt = usersDB.prepare('SELECT username FROM twitch')
			const rows = getStmt.all() as { username: string }[]

			const usernames = rows.map((row) => row.username)

			const liveNow = await fetchLiveNow(usernames)

			feedsDB.prepare('DELETE FROM twitch').run()

			const insertStmt = feedsDB.prepare('INSERT INTO twitch (username, started_at) VALUES (?, ?)')

			const insertStreams = feedsDB.transaction((liveNow) => {
				for (const live of liveNow) {
					insertStmt.run(live.username, live.started_at)
				}
			})

			insertStreams(liveNow)

			const feed: Feed = {
				twitch: liveNow,
				youtube: null
			}

			return feed
		} catch (err) {
			throw new Error(`Refreshing Twitch feed: ${err}`)
		}
	}

	if (platform === Platform.YouTube) {
		try {
			const stmt = usersDB.prepare('SELECT id FROM youtube')
			const rows = stmt.all() as { id: string }[]
			const channelIds = rows.map((row) => row.id)

			const videos = await fetchFeedVideos(channelIds)

			feedsDB.prepare('DELETE FROM youtube').run()

			const insertStmt = feedsDB.prepare(
				'INSERT INTO youtube (id, username, title, published_at, view_count) VALUES (?, ?, ?, ?, ?)'
			)

			const insertVideos = feedsDB.transaction((videos) => {
				for (const video of videos) {
					insertStmt.run(
						video.id,
						video.username,
						video.title,
						video.published_at,
						video.view_count
					)
				}
			})

			insertVideos(videos)

			const initialFeed = videos.slice(0, 50).sort((a, b) => b.published_at - a.published_at)

			const feed: Feed = {
				twitch: null,
				youtube: initialFeed
			}

			return feed
		} catch (err) {
			throw new Error(`Refreshing YouTube feed: ${err}`)
		}
	}

	throw new Error(`Invalid platform '${platform}'`)
}
