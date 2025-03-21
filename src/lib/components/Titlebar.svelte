<script lang="ts">
	import { onMount } from 'svelte';

	import { getCurrentWindow } from '@tauri-apps/api/window';
	import { openUrl } from '@tauri-apps/plugin-opener';
	import { invoke } from '@tauri-apps/api/core';

	import { error } from './Notification.svelte';

	import { updateWatching, watching } from '$lib/Stores.svelte';

	const appWindow = getCurrentWindow();
	let maximized = $state(false);

	let showInfo = $state(false);
	let loadingInfo = $state(false);

	async function openInBrowser() {
		await openUrl(`https://www.twitch.tv/${watching.username}`);
	}

	async function onHoverStart() {
		showInfo = true;

		const now = new Date();
		const elapsed = now.getTime() - watching.last_update.getTime();
		if (elapsed < 60000) return;

		loadingInfo = true;

		await invoke<Stream>('fetch_stream_info', {
			username: watching.username,
			joiningStream: false
		})
			.then((data) => {
				updateWatching(watching.username, data);
			})
			.catch((err) => {
				error(`Error fetching stream info`, err);
			});

		loadingInfo = false;
	}

	function onHoverEnd() {
		showInfo = false;
	}

	onMount(async () => {
		await getCurrentWindow().onResized(async () => {
			maximized = await appWindow.isMaximized();
		});
	});
</script>

<header data-tauri-drag-region class="flex w-full bg-violet-800 min-h-8">
	<div class="w-32 min-w-32"></div>

	{#if watching.username}
		<div class="flex flex-1 justify-center gap-2">
			<button class="text-lg font-bold" data-tauri-drag-region>
				{watching.username}
			</button>

			{#if watching.url}
				<div
					role="tooltip"
					class="px-2 hover:bg-neutral-700 flex items-center"
					onmouseenter={onHoverStart}
					onmouseleave={onHoverEnd}
				>
					<div class="relative inline-block">
						<div class="px-1 hover:bg-neutral-700 flex items-center">
							<svg
								xmlns="http://www.w3.org/2000/svg"
								width="1.0em"
								height="1.0em"
								viewBox="0 0 2048 2048"
								><!-- Icon from Fluent UI MDL2 by Microsoft Corporation - https://github.com/microsoft/fluentui/blob/master/packages/react-icons-mdl2/LICENSE --><path
									fill="currentColor"
									d="M960 0q132 0 255 34t229 97t194 150t150 194t97 230t35 255t-34 255t-97 229t-150 194t-194 150t-230 97t-255 35t-255-34t-229-97t-194-150t-150-194t-97-229T0 960q0-132 34-255t97-229t150-194t194-150t229-97T960 0m64 768H896v640h128zm0-256H896v128h128z"
								/></svg
							>
						</div>

						{#if showInfo}
							{#if !loadingInfo}
								<div class="absolute z-50 right-0 top-6 w-128 h-32" style="user-select: text;">
									<div
										class="relative flex gap-2 w-full h-full bg-neutral-800 shadow-lg rounded-md border border-white/20"
									>
										<img
											src={watching.boxart}
											alt="Game Boxart"
											class="object-cover aspect-ratio h-full"
										/>

										<div class="flex flex-col py-1 mr-2 text-sm">
											<button
												title="Open in browser"
												class="font-bold underline hover:text-blue-300 cursor-pointer"
												onclick={openInBrowser}
											>
												{watching.title}
											</button>

											<div class="flex-1"></div>

											<div>
												{watching.started_at}
											</div>

											<div>
												{watching.view_count} watching
											</div>

											<p
												title={watching.game}
												class="italic overflow-hidden text-ellipsis truncate"
											>
												{watching.game}
											</p>
										</div>
									</div>
								</div>
							{:else}
								<div
									class="absolute z-50 right-0 top-6 w-96 max-w-128 h-32"
									style="user-select: text;"
								>
									<div
										class="relative flex gap-2 w-full h-full bg-neutral-800 shadow-lg rounded-md border border-white/20 animate-pulse"
									></div>
								</div>
							{/if}
						{/if}
					</div>
				</div>
			{/if}
		</div>
	{:else}
		<div class="flex-1"></div>
	{/if}

	<div class="flex h-full">
		<button
			aria-label="Minimize"
			title="Minimize"
			onclick={() => appWindow.minimize()}
			class="px-2 hover:bg-neutral-700"
		>
			<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 2048 2048"
				><!-- Icon from Fluent UI MDL2 by Microsoft Corporation - https://github.com/microsoft/fluentui/blob/master/packages/react-icons-mdl2/LICENSE --><path
					fill="currentColor"
					d="M2048 819v205H0V819z"
				/></svg
			>
		</button>

		<button
			aria-label="Maximize"
			title={maximized ? 'Restore window' : 'Maximize window'}
			onclick={() => appWindow.toggleMaximize()}
			class="px-2 hover:bg-neutral-700"
		>
			{#if maximized}
				<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 2048 2048"
					><!-- Icon from Fluent UI MDL2 by Microsoft Corporation - https://github.com/microsoft/fluentui/blob/master/packages/react-icons-mdl2/LICENSE --><path
						fill="currentColor"
						d="M1024 1657L25 658l121-121l878 878l878-878l121 121z"
					/></svg
				>
			{:else}
				<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 2048 2048"
					><!-- Icon from Fluent UI MDL2 by Microsoft Corporation - https://github.com/microsoft/fluentui/blob/master/packages/react-icons-mdl2/LICENSE --><path
						fill="currentColor"
						d="m1902 1511l-878-878l-878 878l-121-121l999-999l999 999z"
					/></svg
				>
			{/if}
		</button>

		<button
			aria-label="Close"
			title="Close"
			onclick={() => appWindow.close()}
			class="px-2 hover:bg-red-600"
		>
			<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 2048 2048"
				><!-- Icon from Fluent UI MDL2 by Microsoft Corporation - https://github.com/microsoft/fluentui/blob/master/packages/react-icons-mdl2/LICENSE --><path
					fill="currentColor"
					d="m1169 1024l879 879l-145 145l-879-879l-879 879L0 1903l879-879L0 145L145 0l879 879L1903 0l145 145z"
				/></svg
			>
		</button>
	</div>
</header>

<style>
	header {
		-webkit-app-region: drag;
		user-select: none;
	}

	button {
		-webkit-app-region: no-drag;
	}
</style>
