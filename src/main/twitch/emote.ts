import { emotesDB } from '../database/dbs'
import { SubscriptionProduct } from './query'

const TWITCH_EMOTES_CDN: string = 'https://static-cdn.jtvnw.net/emoticons/v2'
const SEVENTV_API: string = 'https://7tv.io/v3'
const BETTERTV_API: string = 'https://api.betterttv.net/3'

type EmoteDB = {
	name: string
	url: string
	width: number
	height: number
}

export async function getUserEmotes(username: string): Promise<Record<string, Emote>> {
	const query = 'SELECT name, url, width, height FROM twitch WHERE username = ?'

	const rows = emotesDB.prepare(query).all(username) as EmoteDB[]

	const emotes: Record<string, Emote> = {}

	for (const row of rows) {
		const emote: Emote = {
			n: row.name,
			u: row.url,
			w: row.width,
			h: row.height
		}

		emotes[row.name] = emote
	}

	return emotes
}

export async function updateUserEmotes(
	username: string,
	emotes: Record<string, Emote>
): Promise<void> {
	if (!emotes || Object.keys(emotes).length === 0) {
		return
	}

	const query = 'DELETE FROM twitch WHERE username = ?'
	emotesDB.prepare(query).run(username)

	const insertStmt = emotesDB.prepare(
		'INSERT INTO twitch (username, name, url, width, height) VALUES (?, ?, ?, ?, ?)'
	)

	const insertEmotes = emotesDB.transaction((emotes: Record<string, Emote>) => {
		Object.entries(emotes).forEach(([name, emote]) => {
			if (name === '' || username === '') return
			insertStmt.run(username, name, emote.u, emote.w, emote.h)
		})
	})

	insertEmotes(emotes)
}

export function parseSubscriptionProducts(
	subscriptionProducts: SubscriptionProduct[]
): Record<string, Emote> {
	const userEmotes: Record<string, Emote> = {}

	for (const product of subscriptionProducts) {
		for (const e of product.emotes) {
			const name = e.token
			const url = `${TWITCH_EMOTES_CDN}/${e.id}/default/dark/1.0`

			const emote: Emote = {
				n: name,
				u: url,
				w: 28,
				h: 28
			}

			userEmotes[name] = emote
		}
	}

	return userEmotes
}

type BetterTTVResponse = {
	channelEmotes: BetterTTVEmote[]
	sharedEmotes: BetterTTVEmote[]
}

type BetterTTVEmote = {
	id: string
	code: string
	width?: number
	height?: number
}

export async function fetchBetterTTVEmotes(id: string): Promise<Record<string, Emote>> {
	const response = await fetch(`${BETTERTV_API}/cached/users/twitch/${id}`)

	if (!response.ok) {
		throw new Error(`BetterTTV request returned ${response.status}`)
	}

	const json: BetterTTVResponse = await response.json()

	const rawEmotes = [...json.channelEmotes, ...json.sharedEmotes]

	const emotes: Record<string, Emote> = {}

	for (const e of rawEmotes) {
		const name = e.code
		const url = `https://cdn.betterttv.net/emote/${e.id}/1x`
		const emote: Emote = {
			n: name,
			u: url,
			w: e.width ?? 28,
			h: e.height ?? 28
		}

		emotes[name] = emote
	}

	return emotes
}

type SevenTVResponse = {
	emote_set: {
		emotes: {
			name: string
			data: {
				host: {
					url: string
					files: {
						name: string
						width: number
						height: number
						format: string
					}[]
				}
			}
		}[]
	}
}

export async function fetch7TVEmotes(id: string): Promise<Record<string, Emote>> {
	const response = await fetch(`${SEVENTV_API}/users/twitch/${id}`)

	if (!response.ok) {
		throw new Error(`7TV request returned ${response.status}`)
	}

	const json: SevenTVResponse = await response.json()

	const emotes: Record<string, Emote> = {}

	for (const e of json.emote_set.emotes) {
		const name = e.name
		const host = e.data.host

		// Assign a priority to each file format
		const priority = (format: string): number => {
			switch (format.toUpperCase()) {
				case 'AVIF':
					return 0
				case 'WEBP':
					return 1
				case 'PNG':
					return 2
				case 'GIF':
					return 3
				default:
					return -1
			}
		}

		// Find the file with the highest priority (lowest number)
		let bestPriority: number | null = null
		let bestFile: {
			name: string
			width: number
			height: number
			format: string
		} | null = null

		for (const file of host.files) {
			if (bestPriority === null || priority(file.format) > bestPriority) {
				bestPriority = priority(file.format)
				bestFile = file
			}
		}

		if (bestFile === null) {
			continue
		}

		const newEmote: Emote = {
			n: name,
			u: `https:${host.url}/${bestFile.name}`,
			w: bestFile.width,
			h: bestFile.height
		}

		emotes[name] = newEmote
	}

	return emotes
}
