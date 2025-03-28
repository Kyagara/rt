<script lang="ts">
	import '../app.css';

	import 'simplebar';
	import 'simplebar/dist/simplebar.css';

	import Notification from '$lib/components/Notification.svelte';
	import Sidebar from '$lib/components/Sidebar.svelte';
	import Titlebar from '$lib/components/Titlebar.svelte';

	let { children } = $props();

	// Prevents the context menu from appearing
	function handleRightClick(event: MouseEvent) {
		event.preventDefault();
		event.stopPropagation();
		return;
	}
</script>

<div
	role="application"
	class="flex h-screen w-screen flex-col bg-neutral-950 text-white"
	oncontextmenu={handleRightClick}
>
	<Titlebar />

	<div class="flex min-h-0 flex-1">
		<Sidebar />

		<main class="flex h-full min-h-0 w-full overflow-auto">
			{@render children()}
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
</style>
