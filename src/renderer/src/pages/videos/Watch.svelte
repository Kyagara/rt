<script lang="ts">
	import { onMount } from 'svelte'

	import YouTubePlayer from '$lib/components/players/YouTube.svelte'
	import { notify } from '$lib/components/Notification.svelte'

	import { Platform } from '$shared/enums'
	import { getAvatarUrl } from '$lib/index'

	let player = $state() as WatchPageVideo
	let usingEmbed = $state(false)
	let autoPlay = $state(true)

	let subscribed = $state(false)

	let loading = $state(true)

	function copyURL() {
		navigator.clipboard.writeText(`https://youtu.be/${player.id}`)
		notify('URL copied to clipboard')
	}

	function toggleEmbed() {
		usingEmbed = !usingEmbed
		notify('Switched to ' + (usingEmbed ? 'player' : 'embed'))
	}

	async function handleSubscription() {
		try {
			if (subscribed) {
				const username = await window.user.remove(Platform.YouTube, player.channel.name)
				notify(`Unsubscribed from ${username}`)
				subscribed = false
			} else {
				const user = await window.user.add(Platform.YouTube, null, player.channel.id)
				notify(`Subscribed to ${user.username}`)
				subscribed = true
			}
		} catch (err) {
			notify('Error updating subscription', err)
		}
	}

	async function getPossibleUser() {
		if (!player.channel.id) return

		const user = await window.user.get(Platform.YouTube, null, player.channel.id)
		if (user) {
			subscribed = true
			if (player.channel.avatar && !user.avatar) return

			player.channel.avatar = getAvatarUrl(Platform.YouTube, player.channel.name, user.avatar)
		}
	}

	onMount(async () => {
		const routeURL = new URL(window.location.href)
		const searchParams = routeURL.searchParams.get('id')

		if (!searchParams) {
			notify('No video ID found')
			return
		}

		let videoID = searchParams
		if (searchParams.startsWith('watch?v=')) {
			videoID = searchParams.replace('watch?v=', '')
		}

		const settings = localStorage.getItem('settings')
		if (settings) {
			const data: Settings = JSON.parse(settings)
			autoPlay = data.videos.autoplay
			usingEmbed = data.videos.useEmbed
		}

		if (usingEmbed) {
			try {
				const data = await window.video.get(videoID, false)
				player = data

				await getPossibleUser()
			} catch (err) {
				notify('Error fetching video', err)
			}

			loading = false
			return
		}

		try {
			const data = await window.video.get(videoID, true)
			player = data
			usingEmbed = false

			await getPossibleUser()
		} catch (err) {
			notify('Error fetching player', err)
		}

		loading = false
	})
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
				{#if player && player.id}
					<YouTubePlayer {player} {usingEmbed} {autoPlay} />
				{/if}
			{/key}
		</div>

		<div class="flex w-full flex-col gap-4 p-2">
			<div class="flex gap-4">
				<div class="flex flex-col gap-4">
					<div class="flex-col">
						<h1 class="text-lg font-bold">{player.info.title}</h1>

						<span class="text-xs">
							{player.isLive ? 'Live now' : player.info.published_date_txt} - {player.info.view_count.toLocaleString()}
							views
						</span>
					</div>

					<div class="flex items-center gap-2">
						<img
							src={player.channel.avatar}
							alt={player.channel.name}
							width={48}
							height={64}
							class="border border-white/25"
						/>

						<span class="font-semibold">
							{player.channel.name}
						</span>

						<button
							class="cursor-pointer border border-white/25 p-1 px-2 hover:bg-red-400/80"
							onclick={async () => await handleSubscription()}
						>
							{subscribed ? 'Remove user' : 'Add user'}
						</button>
					</div>
				</div>

				<div class="flex-1"></div>

				<div>
					<button
						class="cursor-pointer bg-neutral-800 p-2 hover:bg-neutral-600"
						title="Copy video URL"
						onclick={copyURL}
					>
						<svg
							xmlns="http://www.w3.org/2000/svg"
							width="1.1rem"
							height="1.1rem"
							viewBox="0 0 2048 2048"
							><!-- Icon from Fluent UI MDL2 by Microsoft Corporation - https://github.com/microsoft/fluentui/blob/master/packages/react-icons-mdl2/LICENSE --><path
								fill="currentColor"
								d="M1920 805v1243H640v-384H128V0h859l384 384h128zm-384-37h165l-165-165zM640 384h549L933 128H256v1408h384zm1152 512h-384V512H768v1408h1024z"
							/></svg
						>
					</button>

					<button
						class="cursor-pointer bg-neutral-800 p-2 hover:bg-neutral-600"
						title={usingEmbed ? 'Switch to player' : 'Switch to embed'}
						onclick={toggleEmbed}
					>
						<svg
							xmlns="http://www.w3.org/2000/svg"
							width="1.1rem"
							height="1.1rem"
							viewBox="0 0 2048 2048"
							><!-- Icon from Fluent UI MDL2 by Microsoft Corporation - https://github.com/microsoft/fluentui/blob/master/packages/react-icons-mdl2/LICENSE --><path
								fill="currentColor"
								d="M2048 1408v128H250l163 163l-90 90L6 1472l317-317l90 90l-163 163zm-413-605l163-163H0V512h1798l-163-163l90-90l317 317l-317 317z"
							/></svg
						>
					</button>
				</div>
			</div>

			<hr class="w-2/3 border-neutral-400/25" />

			<div class="flex flex-col gap-2">
				{#if player.info.description}
					<!-- eslint-disable-next-line svelte/no-at-html-tags -->
					{@html player.info.description}
				{:else}
					<span class="text-xs">No description available</span>
				{/if}
			</div>
		</div>
	{/if}
</div>
