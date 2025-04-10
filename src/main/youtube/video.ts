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

const yt = Innertube.create({ retrieve_player: true })

export async function fetchVideo(videoID: string): Promise<WatchPageVideo> {
	if (!videoID) throw new Error('Video ID not provided')

	const client = await yt

	const videoInfo = await client.getInfo(videoID)

	const info = {
		published_date_txt: videoInfo.primary_info?.relative_date.toString() ?? '',
		description: videoInfo.secondary_info?.description.toHTML() ?? '',
		title: videoInfo.primary_info?.title.toString() ?? '',
		view_count: Number(videoInfo.primary_info?.view_count?.original_view_count ?? 0)
	}

	const channel: WatchPageVideoChannel = {
		id: videoInfo.basic_info.channel?.id ?? '',
		name: videoInfo.basic_info.channel?.name ?? '',
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
		dash: isLive
			? undefined
			: await videoInfo.toDash(undefined, undefined, { captions_format: 'vtt' }),
		live,
		info,
		channel
	}

	return video
}
