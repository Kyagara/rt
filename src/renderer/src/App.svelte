<script lang="ts">
	import { onMount } from 'svelte'

	import 'simplebar'
	import 'simplebar/dist/simplebar.css'

	import Notification from '$lib/components/Notification.svelte'
	import Sidebar from '$lib/components/Sidebar.svelte'
	import Titlebar from '$lib/components/Titlebar.svelte'

	import { changeView, currentView } from '$lib/state/View.svelte'
	import { defaultSettings } from './lib'
	import { View } from '$shared/enums'

	// Pages
	import Videos from './pages/videos/Videos.svelte'
	import VideoWatchPage from './pages/videos/Watch.svelte'

	import Streams from './pages/streams/Streams.svelte'
	import StreamWatchPage from './pages/streams/Watch.svelte'

	import Users from './pages/Users.svelte'
	import Settings from './pages/Settings.svelte'

	// Prevent the context menu from appearing
	function handleContextMenu(event: MouseEvent) {
		event.preventDefault()
		event.stopPropagation()
		return
	}

	onMount(() => {
		const storedSettings = localStorage.getItem('settings')
		if (!storedSettings) {
			localStorage.setItem('settings', JSON.stringify(defaultSettings()))
			return
		}

		const url = URL.parse(window.location.href)
		const view = url.searchParams.get('view') as View
		const path = url.searchParams.get('path') ?? ''

		const lastView = localStorage.getItem('lastView') as View

		if (lastView !== view && !path) {
			changeView(lastView)
			return
		}

		changeView(view, true, path)
	})
</script>

<div
	role="application"
	class="flex h-screen w-screen flex-col bg-neutral-950 text-white"
	oncontextmenu={handleContextMenu}
>
	<Titlebar />

	<div class="flex min-h-0 flex-1">
		<Sidebar />

		<main class="flex h-full min-h-0 w-full overflow-auto">
			{#if currentView.id === View.Videos}
				{#if currentView.route.startsWith('/videos/watch')}
					<VideoWatchPage />
				{:else}
					<Videos />
				{/if}
			{:else if currentView.id === View.Streams}
				{#if currentView.route.startsWith('/streams/watch')}
					<StreamWatchPage />
				{:else}
					<Streams />
				{/if}
			{:else if currentView.id === View.Users}
				<Users />
			{:else}
				<Settings />
			{/if}
		</main>
	</div>

	<Notification />
</div>

<style>
	:global(html) {
		user-select: none;
		-webkit-user-select: none;
		-ms-user-select: none;
	}

	:global(.simplebar-scrollbar) {
		transition: opacity 0.2s ease-in-out;
	}

	:global(.simplebar-scrollbar::before) {
		background-color: #ffffff;
	}

	:global(media-player) {
		height: 100%;
		max-height: 100%;
	}

	:global(media-provider iframe) {
		height: 100%;
		max-height: 100%;
		max-width: 100%;
		object-fit: contain;
	}

	:global(media-player video) {
		height: 100%;
		max-height: 100%;
		max-width: 100%;
		object-fit: contain;
	}
</style>
