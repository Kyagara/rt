<script lang="ts">
	import { onMount } from 'svelte'

	import Grid from '$lib/components/Grid.svelte'
	import FeedHeader from '$lib/components/FeedHeader.svelte'
	import { notify } from '$lib/components/Notification.svelte'

	import { changeView } from '$lib/state/View.svelte'
	import { timeAgo } from '$lib'
	import { Platform, View } from '$shared/enums'

	let feed = $state([]) as FeedVideo[]

	let loading = $state(true)
	let last_published_at = $state(0)
	let hasMore = $state(true)
	let loader = $state() as HTMLDivElement

	async function fetchMore() {
		if (loading || !hasMore) return

		loading = true

		const req: { platform: Platform; lastPublishedAt?: number } = {
			platform: Platform.YouTube,
			lastPublishedAt: last_published_at
		}

		if (feed.length) {
			req.lastPublishedAt = feed[feed.length - 1].published_at
		} else {
			delete req.lastPublishedAt
		}

		try {
			const data = await window.feed.get(Platform.YouTube, req.lastPublishedAt)
			feed = [...feed, ...data.youtube]
			if (data.youtube.length < 50) {
				hasMore = false
			}
		} catch {
			notify('Error getting videos')
		}

		loading = false
	}

	async function refreshVideos() {
		loading = true
		try {
			const data = await window.feed.refresh(Platform.YouTube)
			if (data.youtube.length < 50) {
				hasMore = false
			} else {
				hasMore = true
			}

			last_published_at = data.youtube[data.youtube.length - 1].published_at
			feed = data.youtube
		} catch {
			notify('Error refreshing videos')
		}
		loading = false
	}

	function getViewCount(viewCount: string) {
		const num = Number(viewCount).toLocaleString()

		if (!num || num === '0') {
			return ''
		}

		return `- ${num} views`
	}

	function handleMouseWheelClick(event: MouseEvent, videoID: string) {
		// Middle mouse button
		if (event.button === 1) {
			window.main.newWindow(`/videos/watch?id=${videoID}`)
		}
	}

	onMount(() => {
		window.feed.get(Platform.YouTube).then((data) => {
			if (data && data.youtube) {
				feed = [...feed, ...data.youtube]

				if (data.youtube.length < 50) {
					hasMore = false
				}
			}

			loading = false
		})

		const observer = new IntersectionObserver(
			async (entries) => {
				if (entries[0].isIntersecting) {
					await fetchMore()
				}
			},
			{ root: null, threshold: 0.7 }
		)

		if (loader) {
			observer.observe(loader)
		}

		return () => {
			observer.disconnect()
		}
	})
</script>

<div data-simplebar data-simplebar-auto-hide="false" class="flex h-full w-full">
	<div class="flex h-full w-full flex-col">
		<FeedHeader refreshFeed={refreshVideos} loading={() => loading} />

		<hr class="w-full border-gray-700 pb-2" />

		{#if !loading && feed.length === 0}
			<span class="flex w-full justify-center text-lg font-medium">No videos found</span>
		{:else}
			<Grid>
				{#each feed as video, index (index)}
					<button
						onmousedown={(event: MouseEvent) => handleMouseWheelClick(event, video.id)}
						onclick={() => changeView(View.Videos, true, `/watch?id=${video.id}`)}
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
	</div>

	<div>
		<div class="h-2" bind:this={loader}></div>
	</div>
</div>
