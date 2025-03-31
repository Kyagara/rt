<script lang="ts">
	import { onMount } from 'svelte';

	import markdownit from 'markdown-it';

	import YouTubePlayer from '$lib/components/players/YouTube.svelte';
	import { notify } from '$lib/components/Notification.svelte';

	import { changeView } from '$lib/state/View.svelte';
	import { command, getAvatarUrl, Platform } from '$lib';

	let player = $state() as WatchPageVideo;
	let usingEmbed = $state(true);

	let subscribed = $state(false);

	let loading = $state(true);

	const md = markdownit({
		typographer: true,
		html: true,
		breaks: true
	});

	function getDescription() {
		return md.render(player.metadata.description);
	}

	async function handleSubscription() {
		try {
			if (subscribed) {
				await command('remove_user', {
					platform: Platform.YouTube,
					username: player.channel.name
				}).then(() => {
					subscribed = false;
					notify(`Unsubscribed from ${player.channel.name}`);
				});
			} else {
				await command('add_user', {
					platform: Platform.YouTube,
					username: player.channel.name,
					id: player.channel.id
				}).then(() => {
					subscribed = true;
					notify(`Subscribed to ${player.channel.name}`);
				});
			}
		} catch (err) {
			notify(`Error subscribing to ${player.channel.name}: ${err}`);
		}
	}

	onMount(async () => {
		const routeURL = new URL(window.location.href);
		const searchParams = routeURL.searchParams.get('id')!;
		let videoID = searchParams;
		if (searchParams.startsWith('watch?v=')) {
			videoID = searchParams.replace('watch?v=', '');
		}

		changeView('videos', false);

		await command<WatchPageVideo>('fetch_video', { videoId: videoID })
			.then(async (data) => {
				if (!data) return;
				player = data;

				await command<User>('get_user', {
					platform: Platform.YouTube,
					username: player.channel.name
				}).then((user) => {
					if (user) {
						subscribed = true;
						if (player.channel.avatar && !user.avatar) return;

						player.channel.avatar = getAvatarUrl(
							Platform.YouTube,
							player.channel.name,
							user.avatar
						);
					}
				});

				if (data.videoFormats.length === 0) return;
				usingEmbed = false;
			})
			.catch((err) => {
				if (err) {
					notify('Error fetching player');
				}

				usingEmbed = true;
			});

		player.id = videoID;
		loading = false;
	});
</script>

<div data-simplebar class="flex h-full w-full flex-col">
	{#if loading}
		<div
			class="flex h-[calc(100vh-2rem)] max-h-[calc(100vh-2rem)] flex-col items-center justify-center"
		>
			<div
				class="h-32 w-32 animate-spin rounded-full border-t-2 border-b-2 border-neutral-400/25"
			></div>
		</div>
	{:else}
		<div class="h-[calc(100vh-2rem)] max-h-[calc(100vh-2rem)] w-full bg-black">
			{#key usingEmbed}
				<YouTubePlayer {player} {usingEmbed} />
			{/key}
		</div>

		<div class="flex w-full flex-col gap-4 p-2">
			<div class="flex gap-4">
				<div class="flex flex-col gap-2">
					<div class="flex-col">
						<h1 class="text-lg font-bold">{player.metadata.title}</h1>

						<span class="text-xs">
							{player.metadata.publishedDateTxt}
							-
							{player.metadata.viewCount ? `${player.metadata.viewCount} views` : ''}
						</span>
					</div>

					<div class="flex items-center gap-2">
						<img src={player.channel.avatar} alt={player.channel.name} width={48} height={64} />

						<span class="font-semibold">
							{player.channel.name}
						</span>

						<button
							class="cursor-pointer rounded-sm p-1 px-2 hover:bg-neutral-400/50"
							onclick={async () => await handleSubscription()}
						>
							{subscribed ? 'SUBSCRIBED' : 'SUBSCRIBE'}
						</button>
					</div>
				</div>

				<div class="flex-1"></div>

				<div>
					<button
						class="rounded-md bg-neutral-800 p-1 hover:bg-neutral-600"
						onclick={() => (usingEmbed = !usingEmbed)}
					>
						{usingEmbed ? 'Switch to player' : 'Switch to embed'}
					</button>
				</div>
			</div>

			<hr class="w-2/3 border-neutral-400/25" />

			<div class="flex flex-col gap-2">
				{#if player.metadata.description}
					<!-- eslint-disable-next-line svelte/no-at-html-tags -->
					{@html getDescription()}
				{:else}
					<span class="text-xs">No description available</span>
				{/if}
			</div>
		</div>
	{/if}
</div>
