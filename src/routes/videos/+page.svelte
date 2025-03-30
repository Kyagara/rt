<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';

	import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';

	import Grid from '$lib/components/Grid.svelte';

	import { command, Platform, timeAgo } from '$lib';

	let feed = $state([]) as YouTubeVideo[];

	let loading = $state(false);
	let last_published_at = $state(0);
	let hasMore = $state(true);
	let loader = $state() as HTMLDivElement;

	async function fetchMore() {
		if (loading || !hasMore) return;
		loading = true;

		const req: { platform: Platform; lastPublishedAt?: number } = {
			platform: Platform.YouTube,
			lastPublishedAt: last_published_at
		};

		if (feed.length) {
			req.lastPublishedAt = feed[feed.length - 1].published_at;
		} else {
			delete req.lastPublishedAt;
		}

		const newVideos = await command<Feed>('get_feed', req).then((feed) => {
			if (feed && feed.youtube) {
				return feed.youtube;
			}

			return [] as YouTubeVideo[];
		});

		feed = [...feed, ...newVideos];

		if (newVideos.length < 50) {
			hasMore = false;
		}

		loading = false;
	}

	function getViewCount(viewCount: string) {
		const num = Number(viewCount).toLocaleString();

		if (!num || num === '0') {
			return '';
		}

		return `- ${num} views`;
	}

	async function handleMouseWheelClick(event: MouseEvent, videoID: string) {
		// Middle mouse button
		if (event.button === 1) {
			await command('open_new_window', { url: `/videos/watch?id=${videoID}` });
		}
	}

	onMount(() => {
		fetchMore();

		const appWebview = getCurrentWebviewWindow();
		appWebview.listen<string>('updated_videos', async () => {
			last_published_at = 0;
			hasMore = true;
			feed = [];
			await fetchMore();
		});

		const observer = new IntersectionObserver(
			async (entries) => {
				if (entries[0].isIntersecting) {
					await fetchMore();
				}
			},
			{ root: null, threshold: 0.8 }
		);

		if (loader) {
			observer.observe(loader);
		}

		return () => {
			observer.disconnect();
		};
	});
</script>

<div data-simplebar data-simplebar-auto-hide="false" class="flex h-full w-full p-2">
	{#if !loading && feed.length === 0}
		<span class="text-lg font-medium">No videos found</span>
	{:else}
		<Grid>
			{#each feed as video, index (index)}
				<button
					onmousedown={async (event: MouseEvent) => await handleMouseWheelClick(event, video.id)}
					onclick={() => goto(`/videos/watch?id=${video.id}`)}
					class="flex cursor-pointer flex-col text-left hover:bg-neutral-800"
				>
					<img
						src={`https://i.ytimg.com/vi/${video.id}/mqdefault.jpg`}
						alt={`Video thumbnail for ${video.id}`}
						height="180"
						width="320"
						class="min-h-[110px] object-contain"
					/>

					<div class="flex flex-col gap-1 p-1">
						<span title={video.title} class="text-md font-semibold text-pretty">
							{video.title}
						</span>

						<span class="pb-2 text-xs">
							{video.username}
							{getViewCount(video.view_count)} - {timeAgo(video.published_at)}
						</span>
					</div>
				</button>
			{/each}
		</Grid>
	{/if}

	<div bind:this={loader}></div>
</div>
