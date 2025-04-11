<script lang="ts">
	import { onMount } from 'svelte'

	import { defaultSettings } from '$lib/index'

	let settings: Settings = $state(defaultSettings())

	function reset() {
		const newSettings = defaultSettings()
		localStorage.setItem('settings', JSON.stringify(newSettings))
		settings = newSettings
	}

	function save() {
		localStorage.setItem('settings', JSON.stringify(settings))
	}

	onMount(() => {
		const storedSettings = localStorage.getItem('settings')
		if (!storedSettings) {
			reset()
			return
		}

		settings = JSON.parse(storedSettings)
	})
</script>

<div data-simplebar data-simplebar-auto-hide="false" class="flex h-full w-full">
	<div class="flex h-full w-full flex-col gap-2">
		<div class="flex p-1">
			<button
				onclick={reset}
				class="cursor-pointer border border-gray-600 bg-neutral-900 px-2 py-1.5 hover:bg-neutral-700"
			>
				Reset settings
			</button>
		</div>

		<div class="flex w-full items-center gap-2 p-1">
			<hr class="w-16 border-gray-500" />
			<span class="px-4 text-lg font-medium">Videos</span>
			<hr class="w-full border-gray-500" />
		</div>

		<div class="flex flex-col gap-2 px-2">
			<div class="flex items-center gap-2">
				<input
					type="checkbox"
					onchange={save}
					bind:checked={settings.videos.autoplay}
					class="border border-gray-600 bg-neutral-900 px-3 py-1 focus:ring-2 focus:ring-blue-500 focus:outline-none"
				/>

				Autoplay
			</div>

			<div class="flex items-center gap-2">
				<input
					type="checkbox"
					onchange={save}
					bind:checked={settings.videos.useEmbed}
					class="border border-gray-600 bg-neutral-900 px-3 py-1 focus:ring-2 focus:ring-blue-500 focus:outline-none"
				/>

				Use embed
			</div>
		</div>

		<div class="flex w-full items-center gap-2 p-1">
			<hr class="w-16 border-gray-500" />
			<span class="px-4 text-lg font-medium">Streams</span>
			<hr class="w-full border-gray-500" />
		</div>

		<div class="flex flex-col gap-2 px-2">
			<div
				title="Enables the custom hls.js loader for Twitch streams"
				class="flex items-center gap-2"
			>
				<input
					type="checkbox"
					onchange={save}
					bind:checked={settings.streams.blockAds}
					class="border border-gray-600 bg-neutral-900 px-3 py-1 focus:ring-2 focus:ring-blue-500 focus:outline-none"
				/>

				Block ads
			</div>
		</div>
	</div>
</div>
