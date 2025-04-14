import { View } from '$shared/enums'

const TWITCH_REGEX = /(?:https?:\/\/)?(?:www\.)?twitch\.tv\/([a-zA-Z0-9_]+)/

const YOUTUBE_REGEX =
	/(?:https?:\/\/)?(?:www\.)?(?:youtube\.com\/(?:[^/]+\/.+\/|(?:v|embed|shorts|watch)?\??v=|.*[?&]v=)|youtu\.be\/)([^"&?/\s]{11})/

export function handleURL(url?: string): { view: View; path: string } {
	if (!url) return { view: View.Videos, path: '' }

	if (url.startsWith('/videos/watch')) {
		return { view: View.Videos, path: url.replace('/videos/', '/') }
	} else if (url.startsWith('/streams/watch')) {
		return { view: View.Streams, path: url.replace('/streams/', '/') }
	}

	if (url.startsWith('rt://tw/') || url.startsWith('rt://twitch/')) {
		const username = url.replace(/^rt:\/\/(tw|twitch)\//, '').trim()
		if (!username) return { view: View.Streams, path: '' }
		return { view: View.Streams, path: `/watch?username=${username}` }
	}

	const twMatches = url.match(TWITCH_REGEX)
	if (twMatches && twMatches[1]) {
		const username = twMatches[1]
		if (!username) return { view: View.Streams, path: '' }
		return { view: View.Streams, path: `/watch?username=${username}` }
	}

	if (url.startsWith('rt://yt/') || url.startsWith('rt://youtube/')) {
		const videoID = url.replace(/^rt:\/\/(yt|youtube)\//, '').trim()
		if (!videoID) return { view: View.Videos, path: '' }
		return { view: View.Videos, path: `/watch?id=${videoID}` }
	}

	if (url.includes('www.youtube.com/feed/subscriptions')) {
		return { view: View.Videos, path: '' }
	}

	const ytMatches = url.match(YOUTUBE_REGEX)
	if (ytMatches && ytMatches[1]) {
		const videoID = ytMatches[1]
		if (!videoID) return { view: View.Videos, path: '' }
		return { view: View.Videos, path: `/watch?id=${videoID}` }
	}

	return { view: View.Videos, path: '' }
}

export async function downloadImage(url: string): Promise<Uint8Array> {
	if (url.length === 0) {
		console.error('Image URL is empty')
		return new Uint8Array()
	}

	const response = await fetch(url)

	if (!response.ok) {
		console.error(`Downloading image: ${url}`)
		return new Uint8Array()
	}

	const bytes = await response.arrayBuffer()

	return new Uint8Array(bytes)
}

export function upsertKeyValue(
	obj: Record<string, string | string[]> | undefined,
	keyToChange: string,
	value: string[]
): void {
	if (!obj) {
		return
	}

	const keyToChangeLower = keyToChange.toLowerCase()
	for (const key of Object.keys(obj)) {
		if (key.toLowerCase() === keyToChangeLower) {
			obj[key] = value
			return
		}
	}

	obj[keyToChange] = value
}
