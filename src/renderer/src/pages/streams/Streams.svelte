<script lang="ts">
	import { onMount } from 'svelte'

	import Grid from '$lib/components/Grid.svelte'
	import FeedHeader from '$lib/components/FeedHeader.svelte'
	import { notify } from '$lib/components/Notification.svelte'

	import { streamingFor } from '$lib'
	import { changeView } from '$lib/state/View.svelte'
	import { Platform, View } from '$shared/enums'

	let feed = $state([]) as LiveNow[]

	let loading = $state(true)

	async function refreshStreams() {
		loading = true
		try {
			const data = await window.feed.refresh(Platform.Twitch)
			feed = data.twitch.sort((a, b) => a.username.localeCompare(b.username))
		} catch {
			notify('Error refreshing streams')
		}
		loading = false
	}

	function handleMouseWheelClick(event: MouseEvent, username: string) {
		if (event.button === 1) {
			window.main.newWindow(`/streams/watch?username=${username}`)
		}
	}

	onMount(async () => {
		try {
			const data = await window.feed.get(Platform.Twitch)
			feed = data.twitch.sort((a, b) => a.username.localeCompare(b.username))
		} catch {
			notify('Error getting streams')
		}

		loading = false
	})
</script>

<div data-simplebar data-simplebar-auto-hide="false" class="flex h-full w-full">
	<div class="flex h-full w-full flex-col">
		<FeedHeader refreshFeed={refreshStreams} loading={() => loading} />

		<hr class="w-full border-gray-700 pb-2" />

		{#if !loading && feed.length === 0}
			<span class="flex w-full justify-center text-lg font-medium">No streams found</span>
		{:else}
			<Grid>
				{#each feed as live_now, index (index)}
					<button
						onmousedown={(event: MouseEvent) => handleMouseWheelClick(event, live_now.username)}
						onclick={() => changeView(View.Streams, true, `/watch?username=${live_now.username}`)}
						class="flex cursor-pointer flex-col text-left hover:bg-neutral-800"
					>
						<img
							src={`https://static-cdn.jtvnw.net/previews-ttv/live_user_${live_now.username}-440x248.jpg`}
							alt={`Stream thumbnail for ${live_now.username}`}
							height="248"
							width="440"
							class="min-h-[110px] object-contain"
						/>

						<div class="flex flex-col p-1">
							<span class="text-lg font-bold">{live_now.username}</span>

							<span class="text-sm text-neutral-400">{streamingFor(live_now.started_at)}</span>
						</div>
					</button>
				{/each}
			</Grid>
		{/if}
	</div>
</div>
