import {
	PlaybackAccessTokenQuery,
	PlaybackAccessTokenResponse,
	sendQuery,
	StreamInfoQuery,
	StreamInfoResponse,
	UseLiveQuery,
	UseLiveResponse
} from './query'

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

	let url = `https://usher.ttvnw.net/api/channel/hls/${username}.m3u8`

	const randomNumber = Math.floor(Math.random() * 10_000_000) + 1_000_000

	if (backup) {
		url += `?platform=ios&supported_codecs=h264&player=twitchweb&fast_bread=true&p=${randomNumber}&sig=${streamPlayback.signature}&token=${streamPlayback.value}`
	} else {
		url += `?platform=web&supported_codecs=av1,h265,h264&allow_source=true&player=twitchweb&fast_bread=true&p=${randomNumber}&sig=${streamPlayback.signature}&token=${streamPlayback.value}`
	}

	return url
}

export async function fetchStreamInfo(username: string): Promise<StreamInfo> {
	const gql = StreamInfoQuery.new(username)

	const response = await sendQuery<StreamInfoResponse>(JSON.stringify(gql))

	if (!response.data.user) {
		throw new Error(`User '${username}' not found`)
	}

	if (!response.data.user.stream) {
		throw new Error(`User '${username}' is not live`)
	}

	const stream = response.data.user.stream

	let box_art = 'https://static-cdn.jtvnw.net/ttv-static/404_boxart-144x192.jpg'
	if (stream.game && stream.game.id) {
		const box_art_url = `https://static-cdn.jtvnw.net/ttv-boxart/${stream.game.id}-144x192.jpg`

		try {
			const response = await fetch(box_art_url)
			if (response.redirected) {
				box_art = `https://static-cdn.jtvnw.net/ttv-boxart/${stream.game.id}_IGDB-144x192.jpg`
			} else if (response.status === 200) {
				box_art = box_art_url
			}
		} catch (err) {
			console.error(`Error fetching box art: ${err}`)
		}
	}

	return {
		title: stream.title,
		game: stream.game ? stream.game.name : 'N/A',
		box_art: box_art,
		started_at: stream.createdAt,
		viewer_count: stream.viewersCount
	}
}
