import {
	PlaybackAccessTokenQuery,
	PlaybackAccessTokenResponse,
	sendQuery,
	UseLiveQuery,
	UseLiveResponse
} from './query'

const USHER_API: string = 'https://usher.ttvnw.net/api/channel/hls'

export async function fetchLiveNow(usernames: string[]): Promise<LiveNow[]> {
	if (usernames.length === 0) {
		return []
	}

	const query: UseLiveQuery[] = []

	for (const username of usernames) {
		if (username.length === 0) {
			continue
		}

		query.push(UseLiveQuery.new(username))
	}

	const response = await sendQuery<UseLiveResponse[]>(JSON.stringify(query))

	const liveNow: LiveNow[] = []

	for (const obj of response) {
		if (obj.data.user.stream === null) {
			continue
		}

		const stream = obj.data.user.stream
		const username = obj.data.user.login

		const live: LiveNow = {
			username: username,
			started_at: stream.createdAt
		}

		liveNow.push(live)
	}

	return liveNow
}

export async function fetchStream(username: string, backup: boolean): Promise<string> {
	const gql = [UseLiveQuery.new(username), PlaybackAccessTokenQuery.new(username, backup)]

	const response = await sendQuery<[UseLiveResponse, PlaybackAccessTokenResponse]>(
		JSON.stringify(gql)
	)

	const useLive = response[0]
	if (useLive.data.user.stream === null) {
		return ''
	}

	const streamPlayback = response[1].data.streamPlaybackAccessToken

	let url = `${USHER_API}/${username}.m3u8`

	const randomNumber = Math.floor(Math.random() * 10_000_000) + 1_000_000

	if (backup) {
		url += `?platform=ios&supported_codecs=h264&player=twitchweb&fast_bread=true&p=${randomNumber}&sig=${streamPlayback.signature}&token=${streamPlayback.value}`
	} else {
		url += `?platform=web&supported_codecs=av1,h265,h264&allow_source=true&player=twitchweb&fast_bread=true&p=${randomNumber}&sig=${streamPlayback.signature}&token=${streamPlayback.value}`
	}

	return url
}
