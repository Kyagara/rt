<script lang="ts">
	import { onMount } from 'svelte';

	import 'vidstack/bundle';
	import { MediaPlayerElement } from 'vidstack/elements';
	import type { VideoMimeType } from 'vidstack';

	let { player, usingEmbed }: { player: YoutubePlayer; usingEmbed: boolean } = $props();

	let srcs: { src: string; type: VideoMimeType; width: number; height: number }[] = $state([]);

	let playerEl = $state() as MediaPlayerElement;
	let audioEl = $state() as HTMLAudioElement;

	let audioMuted = false;

	onMount(async () => {
		if (!player.sources || player.sources.length === 0) {
			return;
		} else {
			srcs = player.sources.map((s) => ({
				src: s.url,
				type: s.format,
				width: s.width,
				height: s.height
			}));
		}

		usingEmbed = false;

		if (playerEl) {
			const playerSettings = localStorage.getItem('player-settings');
			if (playerSettings) {
				const settings = JSON.parse(playerSettings);
				audioEl.volume = settings.volume;
			}

			playerEl.addEventListener('auto-play', () => {
				playerEl.play();
				audioEl.play();
			});

			playerEl.addEventListener('playing', () => {
				if (audioEl.paused) {
					audioEl.currentTime = playerEl.currentTime;
					audioEl.play();
				}
			});

			playerEl.addEventListener('play', () => {
				audioEl.play();
			});

			playerEl.addEventListener('pause', () => {
				if (audioEl) {
					audioEl.pause();
				}
			});

			playerEl.addEventListener('waiting', () => {
				audioEl.pause();
			});

			playerEl.addEventListener('media-volume-change-request', () => {
				audioMuted = !(playerEl.volume > 0);
				audioEl.volume = playerEl.volume;
			});

			playerEl.addEventListener('media-mute-request', () => {
				audioMuted = true;
				audioEl.muted = audioMuted;
				audioEl.volume = 0;
			});

			playerEl.addEventListener('media-unmute-request', () => {
				audioMuted = false;
				audioEl.muted = audioMuted;
				audioEl.volume = playerEl.volume;
			});

			playerEl.addEventListener('seeked', () => {
				playerEl.pause();
				audioEl.currentTime = playerEl.currentTime;

				audioEl.addEventListener('timeupdate', function onTimeUpdate() {
					playerEl.play();
					audioEl.removeEventListener('timeupdate', onTimeUpdate);
				});
			});
		}
	});
</script>

{#if usingEmbed}
	<media-player
		storage="player-settings"
		src={`https://youtu.be/${player.id}`}
		autoPlay={true}
		streamType="on-demand"
		style="--plyr-border-radius: 0px;"
	>
		<media-provider></media-provider>

		<media-plyr-layout></media-plyr-layout>
	</media-player>
{:else}
	<media-player
		bind:this={playerEl}
		autoPlay={true}
		storage="player-settings"
		src={srcs}
		style="--plyr-border-radius: 0px;"
	>
		<media-provider></media-provider>

		<media-plyr-layout
			displayDuration={true}
			controls={[
				'play',
				'progress',
				'current-time',
				'mute+volume',
				'settings',
				'pip',
				'fullscreen'
			]}
		></media-plyr-layout>
	</media-player>

	<audio bind:this={audioEl} src={player.audio} preload="metadata" style="display: none"> </audio>
{/if}
