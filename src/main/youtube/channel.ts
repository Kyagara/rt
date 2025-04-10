import { dialog } from 'electron'
import { readFile } from 'node:fs/promises'
import { Innertube } from 'youtubei.js'

import { usersDB } from '../database/dbs'
import { downloadImage } from '../utils'
import { Platform } from '$shared/enums'

const yt = Innertube.create({ retrieve_player: false })

export async function fetchChannelByName(channelName: string): Promise<User> {
	const client = await yt

	const results = await client.search(channelName, { type: 'channel' })

	const channelResult = results.channels.find(
		(item) => item.author.name.toLowerCase() === channelName.toLowerCase()
	)

	if (!channelResult) {
		throw new Error(`Channel '${channelName}' not found`)
	}

	const channelID = channelResult.author.id

	try {
		const channel = await client.getChannel(channelID)

		const avatar = await downloadImage(
			getThumbnailForSize(channel.metadata.avatar?.[0].url ?? '', 70)
		)

		const display_name = channel.metadata.title ?? ''
		const base_url = channel.metadata.vanity_channel_url ?? ''
		const username = base_url.split('/').pop()?.replace('@', '') ?? ''

		return {
			id: channelID,
			username,
			display_name,
			platform: Platform.YouTube,
			avatar
		}
	} catch (err) {
		throw new Error(`Fetching channel '${channelName}': ${err}`)
	}
}

export async function fetchChannelByID(channelID: string): Promise<User> {
	const client = await yt

	try {
		const channel = await client.getChannel(channelID)

		const avatar = await downloadImage(
			getThumbnailForSize(channel.metadata.avatar?.[0].url ?? '', 70)
		)

		const display_name = channel.metadata.title ?? ''
		const base_url = channel.metadata.vanity_channel_url ?? ''
		const username = base_url.split('/').pop()?.replace('@', '') ?? ''

		return {
			id: channelID,
			username,
			display_name,
			platform: Platform.YouTube,
			avatar
		}
	} catch (err) {
		throw new Error(`Fetching channel '${channelID}': ${err}`)
	}
}

export async function importSubscriptions(): Promise<number> {
	const filePaths = await dialog.showOpenDialog({
		properties: ['openFile'],
		filters: [{ name: 'CSV', extensions: ['csv'] }]
	})

	if (filePaths.canceled) return -1

	const filePath = filePaths.filePaths[0]

	const imported: string[] = []

	try {
		const data = await readFile(filePath, { encoding: 'utf8' })

		const lines = data.toString().split('\n')

		for (let i = 1; i < lines.length; i++) {
			const fields = lines[i].split(',')
			if (fields.length != 3) continue
			imported.push(fields[0])
		}
	} catch (err) {
		throw new Error(`Reading subscriptions file: ${err}`)
	}

	const getStmt = usersDB.prepare('SELECT id, username, avatar FROM youtube')
	const rows = getStmt.all() as { id: string }[]

	const filtered = imported.filter((channelID) => !rows.some((row) => row.id === channelID))

	const promises = filtered.map(async (channelID) => {
		try {
			const channel = await fetchChannelByID(channelID)
			return channel
		} catch (err) {
			console.error(`Importing channel '${channelID}': ${err}`)
			return
		}
	})

	const channels = await Promise.all(promises)

	const insertStmt = usersDB.prepare(
		'INSERT INTO youtube (id, username, display_name, avatar) VALUES (?, ?, ?, ?)'
	)

	let i = 0
	const insertChannels = usersDB.transaction(async (channels: (User | undefined)[]) => {
		for (const channel of channels) {
			if (!channel || channel.username === '') return

			insertStmt.run(channel.id, channel.username, channel.display_name, channel.avatar)
			i++
		}
	})

	await insertChannels(channels)

	return i
}

function getThumbnailForSize(url: string, size: number): string {
	return url.replace(/=s\d+/, `=s${size}`)
}
