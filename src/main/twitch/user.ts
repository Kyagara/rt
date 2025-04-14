import { Platform } from '$shared/enums'
import { downloadImage } from '../utils'
import { fetch7TVEmotes, fetchBetterTTVEmotes, parseSubscriptionProducts } from './emote'
import { sendQuery, TurboAndSubUpsellQuery, TurboAndSubUpsellResponse } from './query'

export async function fetchTwitchUser(
	username: string
): Promise<{ user: User; emotes: Record<string, Emote> }> {
	const gql = TurboAndSubUpsellQuery.new(username)

	const response = await sendQuery<TurboAndSubUpsellResponse>(JSON.stringify(gql))

	if (!response.data.user) {
		throw new Error(`User '${username}' not found`)
	}

	const twitchUser = response.data.user

	const userEmotes = parseSubscriptionProducts(twitchUser.subscriptionProducts)

	const user_id = twitchUser.id

	const sevenTV = await fetch7TVEmotes(user_id)
	const betterTTV = await fetchBetterTTVEmotes(user_id)

	const emotes: Record<string, Emote> = { ...userEmotes, ...sevenTV, ...betterTTV }

	const avatar = await downloadImage(twitchUser.profileImageURL)

	const user = {
		id: twitchUser.id,
		username,
		display_name: twitchUser.displayName,
		avatar,
		platform: Platform.Twitch
	}

	return { user, emotes }
}
