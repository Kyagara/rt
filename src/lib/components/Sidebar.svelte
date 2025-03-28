<script lang="ts">
	import { page } from '$app/stores';
	import { fade } from 'svelte/transition';

	import { notify } from './Notification.svelte';

	import { currentView, changeView } from '$lib/state/View.svelte';

	import { command, Platform } from '$lib';

	let path = $state($page.url.pathname);

	let loading = $state(false);

	async function refreshFeed() {
		loading = true;

		const platform = currentView.id === 'streams' ? Platform.Twitch : Platform.YouTube;

		await command('refresh_feed', { platform }).then(() => {
			notify(`Refreshed feed`);
		});

		loading = false;
	}

	async function openNewWindow() {
		await command('open_new_window', { url: `/${currentView.id}` });
	}

	$effect(() => {
		path = $page.url.pathname;
	});
</script>

<aside class="user-select-none flex w-12 min-w-12 flex-col items-center gap-2 bg-neutral-800">
	<div class="flex w-full flex-col items-center">
		<button
			aria-label="Videos"
			title="Videos"
			onclick={() => changeView('videos')}
			disabled={path === '/videos'}
			class="flex w-full flex-col items-center py-2 {path === '/videos'
				? 'opacity-50 duration-100 ease-in-out'
				: 'cursor-pointer hover:bg-neutral-600'}"
		>
			<svg xmlns="http://www.w3.org/2000/svg" width="1.5rem" height="1.5rem" viewBox="0 0 2048 2048"
				><!-- Icon from Fluent UI MDL2 by Microsoft Corporation - https://github.com/microsoft/fluentui/blob/master/packages/react-icons-mdl2/LICENSE --><path
					fill="currentColor"
					d="m2048 1544l-512-256v248H0V512h1536v248l512-256zm-640-904H128v768h1280zm512 71l-384 193v240l384 193z"
				/></svg
			>
		</button>

		<button
			aria-label="Streams"
			title="Streams"
			onclick={() => changeView('streams')}
			disabled={path === '/streams'}
			class="flex w-full flex-col items-center py-2 {path === '/streams'
				? 'opacity-50 duration-100 ease-in-out'
				: 'cursor-pointer hover:bg-neutral-600'}"
		>
			<svg xmlns="http://www.w3.org/2000/svg" width="1.5rem" height="1.5rem" viewBox="0 0 2048 2048"
				><!-- Icon from Fluent UI MDL2 by Microsoft Corporation - https://github.com/microsoft/fluentui/blob/master/packages/react-icons-mdl2/LICENSE --><path
					fill="currentColor"
					d="M1024 779q51 0 95 19t78 53t52 77t20 96q0 51-19 95t-53 78t-77 52t-96 20q-51 0-95-19t-78-53t-52-77t-20-96q0-51 19-95t53-78t77-52t96-20m0 384q29 0 54-11t44-29t30-44t11-55t-11-54t-29-44t-44-30t-55-11t-54 11t-44 29t-30 44t-11 55t11 54t29 44t44 30t55 11m716-855q72 71 127 154t93 174t57 189t20 199q0 101-19 199t-58 189t-93 174t-127 154l-75-75q64-64 113-138t83-156t51-169t18-178q0-90-17-177t-51-170t-83-156t-114-138zM383 383q-64 64-113 138t-84 156t-51 169t-18 178q0 90 17 177t52 170t83 156t114 138l-75 75q-72-71-127-154t-93-174t-57-189t-20-199q0-101 19-199t58-189t93-174t127-154zm1086 196q89 90 136 204t48 241q0 126-47 240t-137 205l-75-75q74-74 113-169t40-201q0-105-39-200t-114-170zm-815 75q-74 74-113 169t-40 201q0 105 39 200t114 170l-75 75q-89-90-136-204t-48-241q0-126 47-240t137-205z"
				/></svg
			>
		</button>

		<button
			aria-label="Users"
			title="Users"
			onclick={() => changeView('users')}
			disabled={currentView.id === 'users'}
			class="flex w-full flex-col items-center py-2 {currentView.id === 'users'
				? 'opacity-50 duration-100 ease-in-out'
				: 'cursor-pointer hover:bg-neutral-600'}"
		>
			<svg xmlns="http://www.w3.org/2000/svg" width="1.5rem" height="1.5rem" viewBox="0 0 2048 2048"
				><!-- Icon from Fluent UI MDL2 by Microsoft Corporation - https://github.com/microsoft/fluentui/blob/master/packages/react-icons-mdl2/LICENSE --><path
					fill="currentColor"
					d="M1397 1550q-21-114-78-210t-141-166t-189-110t-221-40q-88 0-170 23t-153 64t-129 100t-100 130t-65 153t-23 170H0q0-117 35-229t101-207t157-169t203-113q-56-36-100-83t-76-103t-47-119t-17-129q0-106 40-199t109-163T568 40T768 0t199 40t163 109t110 163t40 200q0 66-16 129t-48 119t-75 103t-101 83q99 38 183 100t147 143t105 177t54 202l-57 58zM384 512q0 80 30 149t82 122t122 83t150 30q79 0 149-30t122-82t83-122t30-150q0-79-30-149t-82-122t-123-83t-149-30q-80 0-149 30t-122 82t-83 123t-30 149m1645 941l-557 558l-269-270l90-90l179 178l467-466z"
				/></svg
			>
		</button>
	</div>

	<hr class="w-full border-gray-600" />

	<div class="flex h-full w-full flex-col items-center">
		{#if (currentView.id === 'streams' || currentView.id === 'videos') && !path.includes('watch')}
			<button
				aria-label="Refresh"
				title="Refresh"
				disabled={loading}
				onclick={async () => await refreshFeed()}
				transition:fade={{ duration: 50 }}
				class="flex w-full cursor-pointer flex-col items-center py-2 {loading
					? 'opacity-50'
					: 'hover:bg-neutral-600'}"
			>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="1.5rem"
					height="1.5rem"
					viewBox="0 0 2048 2048"
					><!-- Icon from Fluent UI MDL2 by Microsoft Corporation - https://github.com/microsoft/fluentui/blob/master/packages/react-icons-mdl2/LICENSE --><path
						fill="currentColor"
						d="M1297 38q166 45 304 140t237 226t155 289t55 331q0 141-36 272t-103 245t-160 207t-208 160t-245 103t-272 37q-141 0-272-36t-245-103t-207-160t-160-208t-103-244t-37-273q0-140 37-272t105-248t167-212t221-164H256V0h512v512H640V215q-117 56-211 140T267 545T164 773t-36 251q0 123 32 237t90 214t141 182t181 140t214 91t238 32q123 0 237-32t214-90t182-141t140-181t91-214t32-238q0-150-48-289t-136-253t-207-197t-266-124z"
					/></svg
				>
			</button>
		{/if}

		<div class="flex-1"></div>

		<button
			aria-label="Open new window"
			title="Open new window"
			onclick={async () => await openNewWindow()}
			class="flex w-full cursor-pointer flex-col items-center py-2 hover:bg-neutral-600"
		>
			<svg xmlns="http://www.w3.org/2000/svg" width="1.5rem" height="1.5rem" viewBox="0 0 2048 2048"
				><!-- Icon from Fluent UI MDL2 by Microsoft Corporation - https://github.com/microsoft/fluentui/blob/master/packages/react-icons-mdl2/LICENSE --><path
					fill="currentColor"
					d="M1536 256h384v384h-128V475l-456 456l-91-91l456-456h-165zm0 768l128-128v768H0V512h1280l-128 128H128v896h1408z"
				/></svg
			>
		</button>
	</div>
</aside>
