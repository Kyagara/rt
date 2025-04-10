import { Platform } from '$shared/enums'

const ytAvatarCache = new Map()
const twAvatarCache = new Map()

export function getAvatarUrl(platform: Platform, username: string, avatar: Uint8Array): string {
	const cache = platform === Platform.Twitch ? twAvatarCache : ytAvatarCache
	if (cache.has(username)) {
		return cache.get(username)
	}

	const blob = new Blob([avatar], { type: 'image/png' })
	const url = URL.createObjectURL(blob)

	cache.set(username, url)
	return url
}

export function timeAgo(timestamp: number): string {
	const now = Math.floor(Date.now() / 1000)
	const secondsAgo = now - timestamp

	if (secondsAgo <= 0) return 'just now'

	if (secondsAgo < 60) return `${secondsAgo} second${plural(secondsAgo)} ago`
	const minutesAgo = Math.floor(secondsAgo / 60)

	if (minutesAgo < 60) return `${minutesAgo} minute${plural(minutesAgo)} ago`
	const hoursAgo = Math.floor(minutesAgo / 60)

	if (hoursAgo < 24) return `${hoursAgo} hour${plural(hoursAgo)} ago`

	const daysAgo = Math.floor(hoursAgo / 24)
	if (daysAgo < 30) return `${daysAgo} day${plural(daysAgo)} ago`

	const monthsAgo = Math.floor(daysAgo / 30)
	if (monthsAgo < 12) return `${monthsAgo} month${plural(monthsAgo)} ago`

	const yearsAgo = Math.floor(monthsAgo / 12)
	return `${yearsAgo} year${plural(yearsAgo)} ago`
}

export function streamingFor(startedAt: string): string {
	const diff = new Date().getTime() - new Date(startedAt).getTime()
	const totalSeconds = Math.floor(diff / 1000)
	const hours = Math.floor(totalSeconds / 3600)
	const minutes = Math.floor((totalSeconds % 3600) / 60)
	const seconds = totalSeconds % 60

	const formattedMinutes = minutes.toString().padStart(2, '0')
	const formattedSeconds = seconds.toString().padStart(2, '0')

	return `${hours}:${formattedMinutes}:${formattedSeconds}`
}

function plural(number: number): string {
	if (number > 1) {
		return 's'
	}

	return ''
}
