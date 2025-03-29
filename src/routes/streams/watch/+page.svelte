<script lang="ts">
	import { onMount } from 'svelte';

	import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';

	import TwitchPlayer from '$lib/components/players/Twitch.svelte';
	import Chat from '$lib/components/Chat.svelte';
	import { notify } from '$lib/components/Notification.svelte';

	import { changeView } from '$lib/state/View.svelte';
	import { command } from '$lib';

	let windowLabel = $state('');
	let username = $state('');
	let url = $state('');

	let loading = $state(true);

	let showChat = $state(false);
	let movingMouse = $state(false);

	let movingMouseTimer = $state(0);

	function toggleChat() {
		showChat = !showChat;
	}

	function handleMousemove() {
		movingMouse = true;

		clearTimeout(movingMouseTimer);

		movingMouseTimer = setTimeout(() => {
			movingMouse = false;
		}, 2000);
	}

	onMount(() => {
		const appWebview = getCurrentWebviewWindow();
		windowLabel = appWebview.label;

		appWebview.listen<string>('stream', (event) => {
			switch (event.payload) {
				case 'main':
					notify('No ads detected, switching main stream.');
					break;

				case 'backup':
					notify('Found ads, switching to backup stream.');
					break;
			}
		});

		const routeURL = new URL(window.location.href);
		username = routeURL.searchParams.get('username')!;
		changeView('streams', false);

		command<string>('fetch_stream_playback', { username, backup: false })
			.then((data) => {
				if (!data) {
					return;
				}

				url = data;
				loading = false;
			})
			.catch(() => {
				notify('Stream not found');
				loading = false;
			});

		document.addEventListener('mousemove', handleMousemove);
		return () => {
			document.removeEventListener('mousemove', handleMousemove);
		};
	});
</script>

<div class="flex h-full w-full {loading ? 'flex-col' : ''}">
	{#if loading}
		<div
			class="flex h-[calc(100vh-2rem)] max-h-[calc(100vh-2rem)] flex-col items-center justify-center"
		>
			<div
				class="h-32 w-32 animate-spin rounded-full border-t-2 border-b-2 border-neutral-400/25"
			></div>
		</div>
	{:else if url}
		<div class="flex min-h-0 min-w-0 flex-1">
			<TwitchPlayer {windowLabel} {username} {url} />
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
