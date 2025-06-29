import { Innertube } from 'youtubei.js'
import { fetchVideos } from './rss'

export async function fetchFeedVideos(channelIDs: string[]): Promise<FeedVideo[]> {
	if (!channelIDs || channelIDs.length === 0) return []

	const videoPromises = channelIDs.map(async (channelId) => {
		const newVideos = await fetchVideos(channelId)
		if (!newVideos) {
			console.error(`Error fetching videos for channel '${channelId}'`)
			return []
		}

		return newVideos
	})

	const videoArrays = await Promise.all(videoPromises)
	const videos = videoArrays.flat()
	return videos
}

const ytPlayer = Innertube.create({ retrieve_player: true })
const yt = Innertube.create({ retrieve_player: false })

export async function fetchVideo(
	videoID: string,
	retrievePlayer: boolean
): Promise<WatchPageVideo> {
	if (!videoID) throw new Error('Video ID not provided')

	if (!retrievePlayer) {
		const client = await yt
		const videoInfo = await client.getInfo(videoID)

		const info = {
			published_date_txt: videoInfo.primary_info?.published.toString() ?? '',
			description: modifyRedirects(videoInfo.secondary_info?.description.toHTML() ?? ''),
			title: videoInfo.primary_info?.title.toString() ?? '',
			view_count: Number(
				videoInfo.primary_info?.view_count?.view_count
					.toString()
					.replace(' views', '')
					.replaceAll(',', '') ?? '0'
			)
		}

		const channel: WatchPageVideoChannel = {
			id: videoInfo.secondary_info?.owner?.author.id ?? '',
			name: videoInfo.secondary_info?.owner?.author.name ?? '',
			avatar: videoInfo.secondary_info?.owner?.author.thumbnails[0].url ?? ''
		}

		return {
			id: videoID,
			isLive: false,
			live: {},
			channel,
			info
		}
	}

	const client = await ytPlayer
	const videoInfo = await client.getInfo(videoID)

	const info = {
		published_date_txt: videoInfo.primary_info?.published.toString() ?? '',
		description: modifyRedirects(videoInfo.secondary_info?.description.toHTML() ?? ''),
		title: videoInfo.primary_info?.title.toString() ?? '',
		view_count: Number(
			videoInfo.primary_info?.view_count?.view_count
				.toString()
				.replace(' views', '')
				.replaceAll(',', '') ?? '0'
		)
	}

	const channel: WatchPageVideoChannel = {
		id: videoInfo.secondary_info?.owner?.author.id ?? '',
		name: videoInfo.secondary_info?.owner?.author.name ?? '',
		avatar: videoInfo.secondary_info?.owner?.author.thumbnails[0].url ?? ''
	}

	const live: WatchPageVideoLive = {}

	let isLive = videoInfo.basic_info.is_live ?? false

	if (isLive) {
		if (videoInfo.streaming_data?.hls_manifest_url) {
			live.hls = videoInfo.streaming_data?.hls_manifest_url
			isLive = true
		}

		if (videoInfo.streaming_data?.dash_manifest_url) {
			live.dash = videoInfo.streaming_data?.dash_manifest_url
			isLive = true
		}

		if (!live.dash && !live.hls) {
			isLive = false
		}
	}

	const video: WatchPageVideo = {
		id: videoID,
		isLive,
		dash: isLive ? undefined : await videoInfo.toDash({ captions_format: 'vtt' }),
		live,
		info,
		channel
	}

	return video
}

const REDIRECT_REGEX = new RegExp(
	/(<a\b[^>]*\bhref\s*=\s*)(['"])(https:\/\/www\.youtube\.com\/redirect\?[^'"]+)\2/gi
)

function modifyRedirects(html: string): string {
	const attrs = ` style="text-decoration: underline; color: rgb(59, 130, 246); target='_blank'"`

	return html.replace(REDIRECT_REGEX, (all, prefix, quote, fullRedirectUrl) => {
		const m = fullRedirectUrl.match(/[?&]q=([^&]+)/)
		if (!m) return all

		const realUrl = decodeURIComponent(m[1])
		return prefix + quote + realUrl + quote + attrs
	})
}
