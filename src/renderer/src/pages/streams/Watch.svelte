<script lang="ts">
	import { onDestroy, onMount } from 'svelte'

	import TwitchPlayer from '$lib/components/players/Twitch.svelte'
	import Chat from '$lib/components/Chat.svelte'
	import { notify } from '$lib/components/Notification.svelte'
	import { Platform } from '$shared/enums'

	let username = $state('')
	let url = $state('')
	let streamInfo = $state() as StreamInfo
	let subscribed = $state(false)

	let loading = $state(true)

	let showChat = $state(false)
	let movingMouse = $state(false)

	let updateTimer = $state() as NodeJS.Timeout

	let movingMouseTimer = $state() as NodeJS.Timeout

	function toggleChat() {
		showChat = !showChat
	}

	function handleMousemove() {
		movingMouse = true

		clearTimeout(movingMouseTimer)

		movingMouseTimer = setTimeout(() => {
			movingMouse = false
		}, 2000)
	}

	async function updateStreamInfo() {
		if (!url) return

		try {
			streamInfo = await window.stream.info(username)
		} catch {
			notify('Error updating stream info')
		}
	}

	let elapsedSeconds = $state(0)
	let interval = $state() as NodeJS.Timeout

	function formatTime(seconds: number): string {
		const hours = Math.floor(seconds / 3600)
		const minutes = Math.floor((seconds % 3600) / 60)
		const secs = seconds % 60
		return `${hours}:${minutes.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`
	}

	async function handleSubscription() {
		try {
			if (subscribed) {
				const deleted = await window.user.remove(Platform.Twitch, username)
				notify(`Unsubscribed from ${deleted}`)
				subscribed = false
			} else {
				const deleted = await window.user.add(Platform.Twitch, username)
				notify(`Subscribed to ${deleted.username}`)
				subscribed = true
			}
		} catch (err) {
			notify('Error updating subscription', err)
		}
	}

	async function getPossibleUser() {
		const user = await window.user.get(Platform.Twitch, username)
		if (user) {
			subscribed = true
		}
	}

	onMount(async () => {
		const routeURL = new URL(window.location.href)
		username = routeURL.searchParams.get('username')!

		try {
			const data = await window.stream.get(username, false)
			url = data

			await getPossibleUser()
		} catch {
			notify('Stream not found')
			loading = false
			return
		}

		try {
			streamInfo = await window.stream.info(username)
			updateTimer = setInterval(updateStreamInfo, 300_000)

			const startedAt = new Date(streamInfo.started_at)
			interval = setInterval(() => {
				const now = new Date()
				elapsedSeconds = Math.floor((now.getTime() - startedAt.getTime()) / 1000)
			}, 1000)
		} catch {
			notify('Error fetching stream info')
		}

		loading = false
	})

	onDestroy(() => {
		clearInterval(interval)
		clearInterval(updateTimer)
	})
</script>

<div data-simplebar class="flex h-full w-full flex-col">
	<div class="flex h-full w-full">
		<div
			class="flex max-h-[calc(100vh-2rem)] min-h-[calc(100vh-2rem)] w-full {loading
				? 'flex-col'
				: ''}"
		>
			{#if loading}
				<div
					class="flex h-[calc(100vh-2rem)] max-h-[calc(100vh-2rem)] flex-col items-center justify-center"
				>
					<div
						class="h-32 w-32 animate-spin rounded-full border-t-2 border-b-2 border-neutral-400/25"
					></div>
				</div>
			{:else if url}
				<!-- svelte-ignore a11y_no_static_element_interactions -->
				<div class="flex min-h-0 min-w-0 flex-1 bg-black" onmousemove={handleMousemove}>
					<TwitchPlayer {username} {url} />
				</div>

				<div class="max-w-1/5 min-w-1/5" hidden={!showChat}>
					<Chat {username} {toggleChat} />
				</div>
			{:else}
				<div
					class="flex h-[calc(100vh-2rem)] max-h-[calc(100vh-2rem)] w-full flex-col items-center justify-center"
				>
					<span class="text-lg font-medium">{`${username} is not live`}</span>
				</div>
			{/if}
		</div>

		{#if !loading && url && movingMouse && !showChat}
			<button
				title="Expand chat"
				class="fixed top-8 right-0 z-50 p-2 hover:bg-neutral-700"
				onclick={toggleChat}
			>
				<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 2048 2048"
					><!-- Icon from Fluent UI MDL2 by Microsoft Corporation - https://github.com/microsoft/fluentui/blob/master/packages/react-icons-mdl2/LICENSE --><path
						fill="currentColor"
						d="m1170 146l-879 878l879 878l-121 121l-999-999l999-999zm853 0l-878 878l878 878l-121 121l-999-999l999-999z"
					/></svg
				>
			</button>
		{/if}
	</div>

	{#if !loading && streamInfo}
		<div class="flex w-full flex-col gap-4 p-2">
			<div class="flex items-center justify-between gap-2">
				<h1 class="text-lg font-bold">{streamInfo.title}</h1>

				<span class="text-xs">
					{formatTime(elapsedSeconds)} - {streamInfo.viewer_count} viewers
				</span>
			</div>

			<div class="flex">
				<button
					class="cursor-pointer border border-white/25 p-1 px-2 hover:bg-neutral-400/50"
					onclick={async () => await handleSubscription()}
				>
					{subscribed ? 'SUBSCRIBED' : 'SUBSCRIBE'}
				</button>
			</div>

			<div class="flex items-center gap-2">
				<img src={streamInfo.box_art} alt={streamInfo.game} width={72} height={96} />

				<span class="font-semibold">
					{streamInfo.game}
				</span>
			</div>
		</div>
	{/if}
</div>
