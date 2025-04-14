<script lang="ts">
	import { onMount } from 'svelte'

	import { timeAgo } from '$lib'
	import { currentView } from '$lib/state/View.svelte'
	import { Platform } from '$shared/enums'

	type FeedHeaderProps = {
		refreshFeed: () => Promise<void>
		loading: () => boolean
	}

	let { refreshFeed, loading }: FeedHeaderProps = $props()

	let lastUpdated = $state('')

	function setLastUpdated() {
		const now = new Date()
		const time = Math.floor(now.getTime() / 1000)
		const platform = currentView.id === 'streams' ? Platform.Twitch : Platform.YouTube

		localStorage.setItem(`${platform}:lastFeedUpdate`, time.toString())
		lastUpdated = `${timeAgo(time)}`
		window.scrollTo(0, 0)
	}

	onMount(() => {
		const platform = currentView.id === 'streams' ? Platform.Twitch : Platform.YouTube
		const data = localStorage.getItem(`${platform}:lastFeedUpdate`)

		if (data) {
			lastUpdated = `${timeAgo(Number(data))}`
			return
		}

		lastUpdated = 'N/A'
	})
</script>

<div>
	<div class="flex items-center gap-2 p-1">
		<button
			aria-label="Refresh"
			title="Refresh feed"
			onclick={async () => {
				await refreshFeed()
				setLastUpdated()
			}}
			disabled={loading()}
			class="flex items-center gap-2 border border-gray-600 bg-neutral-900 p-2 {!loading()
				? 'cursor-pointer hover:bg-neutral-700'
				: ''}"
		>
			<span class="text-sm">Updated: {lastUpdated}</span>

			<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 2048 2048"
				><!-- Icon from Fluent UI MDL2 by Microsoft Corporation - https://github.com/microsoft/fluentui/blob/master/packages/react-icons-mdl2/LICENSE --><path
					fill="currentColor"
					d="M1297 38q166 45 304 140t237 226t155 289t55 331q0 141-36 272t-103 245t-160 207t-208 160t-245 103t-272 37q-141 0-272-36t-245-103t-207-160t-160-208t-103-244t-37-273q0-140 37-272t105-248t167-212t221-164H256V0h512v512H640V215q-117 56-211 140T267 545T164 773t-36 251q0 123 32 237t90 214t141 182t181 140t214 91t238 32q123 0 237-32t214-90t182-141t140-181t91-214t32-238q0-150-48-289t-136-253t-207-197t-266-124z"
				/></svg
			>
		</button>
	</div>
</div>
