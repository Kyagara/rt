<script lang="ts" module>
	import { fly, fade } from 'svelte/transition'

	let visible = $state(false)
	let notificationMessage = $state('')

	export function notify(message: string, error?: Error) {
		notificationMessage = message

		visible = true

		if (error) {
			console.error(error)
		}

		setTimeout(() => {
			visible = false
		}, 3000)
	}
</script>

{#if visible}
	<div
		role="alert"
		tabindex="-1"
		in:fly={{ y: 20, duration: 200 }}
		out:fade={{ duration: 100 }}
		class="fixed bottom-4 left-1/2 z-100 -translate-x-1/2 transform rounded-lg bg-black/60 p-2 text-center shadow-lg"
	>
		{notificationMessage}
	</div>
{/if}
