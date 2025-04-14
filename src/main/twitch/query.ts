const GRAPHQL_API: string = 'https://gql.twitch.tv/gql'

const CLIENT_ID: string = 'kimne78kx3ncx6brgo4mv6wki5h1ko'

const TURBO_AND_SUB_UPSELL_QUERY_HASH: string =
	'5dbca380e47e37808c89479f51f789990ec653428a01b76c649ebe01afb3aa7e'

const USE_LIVE_QUERY_HASH: string =
	'639d5f11bfb8bf3053b424d9ef650d04c4ebb7d94711d644afb08fe9a0fad5d9'

export class TurboAndSubUpsellQuery {
	operationName: string
	variables: ChannelLoginVariable
	extensions: QueryExtensions

	constructor(username: string) {
		this.operationName = 'TurboAndSubUpsell'
		this.variables = ChannelLoginVariable.new(username)
		this.extensions = QueryExtensions.new(TURBO_AND_SUB_UPSELL_QUERY_HASH)
	}

	static new(username: string): TurboAndSubUpsellQuery {
		return new TurboAndSubUpsellQuery(username)
	}
}

export type TurboAndSubUpsellResponse = {
	data: {
		user: {
			id: string
			displayName: string
			profileImageURL: string
			subscriptionProducts: SubscriptionProduct[]
		} | null
	}
}

export type SubscriptionProduct = {
	emotes: { id: string; token: string }[]
}

export class UseLiveQuery {
	operationName: string
	variables: ChannelLoginVariable
	extensions: QueryExtensions

	constructor(username: string) {
		this.operationName = 'UseLive'
		this.variables = ChannelLoginVariable.new(username)
		this.extensions = QueryExtensions.new(USE_LIVE_QUERY_HASH)
	}

	static new(username: string): UseLiveQuery {
		return new UseLiveQuery(username)
	}
}

export type UseLiveResponse = {
	data: {
		user: {
			login: string
			stream: {
				createdAt: string
			} | null
		}
	}
}

export class PlaybackAccessTokenQuery {
	operationName: string
	query: string
	variables: PlaybackAccessTokenQueryVariables

	constructor(username: string, backup: boolean) {
		this.operationName = 'PlaybackAccessToken_Template'
		this.query = `query PlaybackAccessToken_Template($login: String!, $isLive: Boolean!, $vodID: ID!, $isVod: Boolean!, $playerType: String!, $platform: String!) {  streamPlaybackAccessToken(channelName: $login, params: {platform: $platform, playerBackend: "mediaplayer", playerType: $playerType}) @include(if: $isLive) {    value    signature   authorization { isForbidden forbiddenReasonCode }   __typename  }  videoPlaybackAccessToken(id: $vodID, params: {platform: $platform, playerBackend: "mediaplayer", playerType: $playerType}) @include(if: $isVod) {    value    signature   __typename  }}`
		this.variables = PlaybackAccessTokenQuery.variables(username, backup)
	}

	static new(username: string, backup: boolean): PlaybackAccessTokenQuery {
		return new PlaybackAccessTokenQuery(username, backup)
	}

	static variables(username: string, backup: boolean): PlaybackAccessTokenQueryVariables {
		return {
			login: username,
			isLive: true,
			isVod: false,
			vodID: '',
			playerType: backup ? 'autoplay' : 'site',
			platform: backup ? 'ios' : 'web'
		}
	}
}

type PlaybackAccessTokenQueryVariables = {
	login: string
	isLive: boolean
	isVod: boolean
	vodID: string
	playerType: string
	platform: string
}

export type PlaybackAccessTokenResponse = {
	data: {
		streamPlaybackAccessToken: {
			value: string
			signature: string
			authorization: {
				isForbidden: boolean
				forbiddenReasonCode: string
			}
		}
	}
}

// Some queries have this variable
class ChannelLoginVariable {
	channelLogin: string

	constructor(username: string) {
		this.channelLogin = username
	}

	static new(username: string): ChannelLoginVariable {
		return new ChannelLoginVariable(username)
	}
}

// Every persistent query has these fields

class QueryExtensions {
	persistedQuery: PersistedQuery

	constructor(hash: string) {
		this.persistedQuery = PersistedQuery.new(hash)
	}

	static new(hash: string): QueryExtensions {
		return new QueryExtensions(hash)
	}
}

class PersistedQuery {
	version: number
	sha256Hash: string

	constructor(hash: string) {
		this.version = 1
		this.sha256Hash = hash
	}

	static new(hash: string): PersistedQuery {
		return new PersistedQuery(hash)
	}
}

export async function sendQuery<T>(body: string): Promise<T> {
	return await fetch(GRAPHQL_API, {
		method: 'POST',
		body: body,
		headers: {
			'Content-Type': 'application/json',
			'Client-ID': CLIENT_ID
		}
	})
		.then((res) => res.json())
		.catch((err) => {
			throw new Error(`Requesting UseLive: ${err}`)
		})
}
