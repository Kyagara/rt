<script lang="ts">
	import 'vidstack/bundle'

	import { onMount } from 'svelte'

	import { MediaPlayerElement } from 'vidstack/elements'
	import type {
		Loader,
		LoaderConfiguration,
		LoaderStats,
		PlaylistLoaderContext,
		LoaderCallbacks
	} from 'hls.js'

	import { notify } from '$lib/components/Notification.svelte'
	import type { HLSProvider, MediaProviderChangeEvent } from 'vidstack'

	let { username, url } = $props()

	let player = $state() as MediaPlayerElement

	interface StreamState {
		usingBackup: boolean
		mainStreamUrl: string | null
		backupStreamUrl: string | null
	}

	let streamState: StreamState = {
		usingBackup: false,
		mainStreamUrl: null,
		backupStreamUrl: null
	}

	class CustomPlaylistLoader implements Loader<PlaylistLoaderContext> {
		context: PlaylistLoaderContext | null = null
		stats: LoaderStats = {
			aborted: false,
			loaded: 0,
			retry: 0,
			total: 0,
			chunkCount: 0,
			bwEstimate: 0,
			loading: { first: 0, start: 0, end: 0 },
			parsing: { start: 0, end: 0 },
			buffering: { first: 0, start: 0, end: 0 }
		}

		async load(
			context: PlaylistLoaderContext,
			_config: LoaderConfiguration,
			callbacks: LoaderCallbacks<PlaylistLoaderContext>
		): Promise<void> {
			if (context.type === 'manifest') {
				try {
					const data = await fetch(context.url).then((response) => {
						if (!response.ok) {
							throw new Error(`Requesting manifest returned ${response.status}`)
						}

						return response.text()
					})

					this.stats.loaded = data.length

					callbacks.onSuccess({ data, url: context.url }, this.stats, context, null)
				} catch (err) {
					callbacks.onError({ code: 0, text: err.toString() }, context, null, this.stats)
				}

				return
			}

			// context.type === 'level'

			await this.proxyStream(context.url)
				.then((playlist) => {
					this.stats.loaded = playlist.length
					callbacks.onSuccess({ data: playlist, url: context.url }, this.stats, context, null)
				})
				.catch((err) => {
					callbacks.onError({ code: 0, text: err.toString() }, context, null, this.stats)
				})
		}

		async proxyStream(url: string): Promise<string> {
			if (!url) {
				throw new Error('No URL provided')
			}

			let response: Response
			try {
				response = await fetch(url)
			} catch (err) {
				throw new Error(`Proxying request: ${err}`)
			}

			let bodyText: string
			try {
				bodyText = await response.text()
			} catch (err) {
				throw new Error(`Reading response body: ${err}`)
			}

			const isMasterPlaylist = bodyText.includes('#EXT-X-STREAM-INF')
			const adDetected = bodyText.includes('stitched-ad')

			const state: StreamState = { ...streamState }

			if (!isMasterPlaylist) {
				if (!state.mainStreamUrl) {
					state.mainStreamUrl = url
				}

				if (adDetected) {
					if (!state.usingBackup) {
						notify('Ad detected, switching to backup stream')

						state.usingBackup = true
					}

					let backupUrl: string
					if (state.backupStreamUrl) {
						backupUrl = state.backupStreamUrl
					} else {
						try {
							backupUrl = await this.fetchBackupStreamUrl(username)
						} catch (err) {
							notify('Error fetching backup stream', err)
							return ''
						}

						state.backupStreamUrl = backupUrl
					}

					try {
						const updatedPlaylist = await this.fetchPlaylistText(backupUrl)

						streamState = state
						return updatedPlaylist
					} catch (err) {
						notify('Error fetching updated backup playlist', err)
						streamState = state
						return ''
					}
				} else if (state.usingBackup) {
					notify('No ad detected, switching back to main stream')

					try {
						const mainPlaylist = await this.fetchMainStream(username, state)

						state.usingBackup = false
						state.backupStreamUrl = null

						streamState = state
						return mainPlaylist
					} catch (err) {
						notify('Error fetching main stream', err)
						streamState = state
						return ''
					}
				}
			}

			streamState = state
			return bodyText
		}

		async fetchPlaylistText(url: string) {
			try {
				const response = await fetch(url)

				if (!response.ok) {
					throw new Error(`Requesting playlist returned ${response.status}`)
				}

				return response.text()
			} catch (err) {
				throw new Error(`Fetching playlist: ${err}`)
			}
		}

		async fetchMainStream(username: string, state: StreamState) {
			if (!state.mainStreamUrl) {
				console.error('Main stream URL not found. Falling back to backup stream.')
				const backupUrl = await this.fetchBackupStreamUrl(username)
				return await this.fetchPlaylistText(backupUrl)
			}

			const body = await this.fetchPlaylistText(state.mainStreamUrl)
			const adDetected = body.includes('stitched-ad')

			if (adDetected) {
				state.usingBackup = true
				return '#EXTM3U\n#EXT-X-ENDLIST\n'
			}

			state.usingBackup = false
			return body
		}

		async fetchBackupStreamUrl(username: string) {
			let url: string

			try {
				url = await window.stream.get(username, true)
			} catch (err) {
				throw new Error(`Fetching backup stream: ${err}`)
			}

			const body = await this.fetchPlaylistText(url)
			const lines = body.split('\n')

			if (lines.length < 5) {
				throw new Error('Backup master playlist is malformed.')
			}

			return lines[4]
		}

		/* eslint-disable-next-line @typescript-eslint/no-empty-function */
		abort(): void {}
		/* eslint-disable-next-line @typescript-eslint/no-empty-function */
		destroy(): void {}
	}

	onMount(() => {
		player.addEventListener('provider-change', (event: MediaProviderChangeEvent) => {
			const detail = event.detail as HLSProvider
			if (detail?.type === 'hls' && detail.config) {
				detail.config.lowLatencyMode = true

				const settings = localStorage.getItem('settings')
				if (settings) {
					const data: Settings = JSON.parse(settings)
					if (data.streams.blockAds) {
						detail.config.pLoader = CustomPlaylistLoader
					}
				}
			}
		})

		player.addEventListener('can-play', () => {
			// seekToLiveEdge() doesn’t work and setting to the duration might cause the buffer to stall
			player.provider?.setCurrentTime(player.duration - 5)
			player.play()
		})
	})
</script>

<media-player
	bind:this={player}
	storage="player-settings"
	streamType="on-demand"
	style="--plyr-border-radius: 0px;"
>
	<media-provider>
		<source src={url} type="application/x-mpegurl" />
	</media-provider>

	<media-plyr-layout
		displayDuration={true}
		controls={['play', 'progress', 'current-time', 'mute+volume', 'settings', 'pip', 'fullscreen']}
	>
	</media-plyr-layout>
</media-player>
