<script lang="ts">
	import { onMount } from 'svelte';

	import YouTubePlayer from '$lib/components/players/YouTube.svelte';
	import { notify } from '$lib/components/Notification.svelte';

	import { changeView } from '$lib/state/View.svelte';
	import { command } from '$lib';

	let player = $state() as YoutubePlayer;
	let usingEmbed = $state(true);

	let loading = $state(true);

	onMount(async () => {
		const routeURL = new URL(window.location.href);
		const videoID = routeURL.searchParams.get('id')!;
		changeView('videos', false);

		await command<YoutubePlayer>('fetch_player', { videoId: videoID })
			.then(async (data) => {
				if (!data) return;

				player = data;
				player.id = videoID;
				usingEmbed = false;
			})
			.catch(() => {
				notify('Error fetching player');
			});

		loading = false;
	});
</script>

<div data-simplebar class="flex h-full w-full flex-col">
	{#if !loading}
		<div class="h-[calc(100vh-2rem)] max-h-[calc(100vh-2rem)] w-full bg-black">
			<YouTubePlayer {player} {usingEmbed} />
		</div>

		<div class="flex p-2">
			<div class="flex flex-1 flex-col gap-2">
				<div class=" flex-col">
					<h1 class="text-lg font-bold">{player.title}</h1>

					<span class="text-xs">
						{player.published_date_txt}
						-
						{player.view_count ? `${player.view_count} views` : ''}
					</span>
				</div>

				<div class="flex items-center gap-2">
					<span>
						{player.channel_name}
					</span>

					<button class="rounded-md bg-red-500 p-1 hover:bg-red-400">Subscribe</button>
				</div>

				<div class="flex flex-col gap-2">
					<span class="text-xs">Description:</span>

					<div>
						{player.description}
					</div>
				</div>
			</div>

			<div>
				<button
					class="rounded-md bg-neutral-800 p-1 hover:bg-neutral-600"
					onclick={() => (usingEmbed = !usingEmbed)}
				>
					{usingEmbed ? 'Switch to player' : 'Switch to embed'}
				</button>
			</div>
		</div>
	{/if}
</div>
