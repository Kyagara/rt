<script lang="ts">
	import { onMount } from 'svelte';

	import { fetch } from '@tauri-apps/plugin-http';

	import 'vidstack/bundle';
	import { MediaPlayerElement } from 'vidstack/elements';
	import type {
		Loader,
		LoaderConfiguration,
		LoaderStats,
		PlaylistLoaderContext,
		LoaderCallbacks
	} from 'hls.js';

	import { command } from '$lib';

	let { windowLabel, username, url } = $props();

	let player = $state() as MediaPlayerElement;

	class HlsTauriPlaylistLoader implements Loader<PlaylistLoaderContext> {
		context: PlaylistLoaderContext | null = null;
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
		};

		load(
			context: PlaylistLoaderContext,
			_config: LoaderConfiguration,
			callbacks: LoaderCallbacks<PlaylistLoaderContext>
		): void {
			if (context.type === 'manifest') {
				fetch(context.url)
					.then((response) => {
						response
							.text()
							.then((data) => {
								this.stats.loaded = data.length;

								const response = {
									data: data,
									url: context.url
								};

								callbacks.onSuccess(response, this.stats, context, null);
							})
							.catch((err) => {
								callbacks.onError({ code: 0, text: err.toString() }, context, null, this.stats);
							});
					})
					.catch((err) => {
						callbacks.onError({ code: 0, text: err.toString() }, context, null, this.stats);
					});

				return;
			}

			// context.type === 'level'

			command<string>('proxy_stream', {
				windowLabel,
				username,
				url: context.url
			})
				.then((data) => {
					if (!data) {
						callbacks.onError({ code: 0, text: 'No data received' }, context, null, this.stats);
						return;
					}

					this.stats.loaded = data.length;

					const response = {
						data: data,
						url: context.url
					};

					callbacks.onSuccess(response, this.stats, context, null);
				})
				.catch((err) => {
					callbacks.onError({ code: 0, text: err.toString() }, context, null, this.stats);
				});
		}

		abort(): void {}
		destroy(): void {}
	}

	onMount(async () => {
		player.addEventListener('provider-change', (event: any) => {
			if (event.detail?.type === 'hls' && event.detail.config) {
				event.detail.config.pLoader = HlsTauriPlaylistLoader;
				event.detail.config.lowLatencyMode = true;
			}
		});

		player.addEventListener('can-play', () => {
			// seekToLiveEdge() doesn’t work and setting to the duration might cause the buffer to stall
			player.provider?.setCurrentTime(player.duration - 5);
			player.play();
		});
	});
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

<style>
	:global(media-player video) {
		max-height: 100%;
		max-width: 100%;
	}
</style>
