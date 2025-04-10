import type { Database } from 'better-sqlite3'

declare global {
	type Migration = {
		version: number
		up(db: Database): void
		description: string
	}

	type User = {
		id: string
		// Used in links like 'twitch.tv/username' or youtube.com/@username'
		username: string
		display_name: string
		platform: Platform
		avatar: Uint8Array
	}

	type Feed = {
		twitch: LiveNow[] | null
		youtube: FeedVideo[] | null
	}

	type LiveNow = {
		username: string
		started_at: string
	}

	type FeedVideo = {
		id: string
		username: string
		title: string
		published_at: number
		view_count: string
	}

	type WatchPageVideo = {
		id: string
		isLive: boolean
		live: WatchPageVideoLive
		channel: Channel
		info: Info
		dash?: string
	}

	type WatchPageVideoChannel = {
		id: string
		name: string
		avatar: string
	}

	type WatchPageVideoLive = {
		hls?: string
		dash?: string
	}

	type WatchPageVideoInfo = {
		title: string
		description: string
		published_date_txt: string
		view_count: number
	}

	type ChatEvent = {
		event: 'message'
		data: ChatMessage
	}

	type ChatMessage = {
		id: number
		// Color
		c: string
		// First message, not used
		f: boolean
		// Name
		n: string
		// Fragments that make up the message
		m: MessageFragment[]
	}

	type MessageFragment = {
		// export type, 0 = text, 1 = emote, 2 = url
		t: number
		// Content
		c: string
		// Emote
		e: Emote
	}

	type Emote = {
		// Name
		n: string
		// URL
		u: string
		// Width
		w: number
		// Height
		h: number
	}
}
