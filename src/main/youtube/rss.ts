import { XMLParser } from 'fast-xml-parser'

const parser = new XMLParser({
	ignoreAttributes: false
})

export async function fetchVideos(channelID: string): Promise<FeedVideo[]> {
	if (!channelID) throw new Error('Channel ID not provided')

	const response = await fetch(`https://www.youtube.com/feeds/videos.xml?channel_id=${channelID}`)

	if (!response.ok) {
		throw new Error(`Requesting videos for channel '${channelID}' returned ${response.status}`)
	}

	const body = await response.text()

	const videos: FeedVideo[] = []

	const jsonObj = parser.parse(body)
	const feed = jsonObj.feed || jsonObj
	const username = feed.title || ''

	const entries = Array.isArray(feed.entry) ? feed.entry : [feed.entry]

	for (const entry of entries) {
		const id = entry['yt:videoId']
		const title = entry.title || ''
		const published_at = Date.parse(entry.published || '') / 1000

		const view_count =
			entry['media:group']?.['media:community']?.['media:statistics']?.['@_views'] ?? '0'

		if (!id || !username) continue

		videos.push({
			id,
			username,
			title,
			published_at,
			view_count
		})
	}

	return videos
}
