<script lang="ts">
	import { onMount } from 'svelte';

	import { Channel } from '@tauri-apps/api/core';
	import { emit } from '@tauri-apps/api/event';

	import SimpleBar from 'simplebar';

	import { command, openURLInBrowser } from '$lib';

	let { username, toggleChat } = $props();

	let messages: ChatMessage[] = $state([]);
	let tempMessages: ChatMessage[] = $state([]);
	let pendingMessages: ChatMessage[] = [];
	let updateScheduled = false;

	let chatContainer = $state() as HTMLDivElement;
	let simpleBarInstance = $state() as HTMLElement;
	let autoScroll = $state(true);

	function handleScroll() {
		const { scrollTop, scrollHeight, clientHeight } = simpleBarInstance;

		if (scrollTop + clientHeight < scrollHeight - 10) {
			autoScroll = false;
			return;
		}

		autoScroll = true;
		if (tempMessages.length > 0) {
			let combined = [...messages, ...tempMessages];
			if (combined.length > 300) {
				combined = combined.slice(combined.length - 300);
			}

			messages = combined;
			tempMessages = [];
		}
	}

	$effect(() => {
		if (autoScroll && simpleBarInstance && messages.length > 0) {
			simpleBarInstance.scrollTop = simpleBarInstance.scrollHeight;
		}
	});

	onMount(() => {
		simpleBarInstance = new SimpleBar(chatContainer).getScrollElement()!;
		simpleBarInstance.addEventListener('scroll', handleScroll);

		messages = [];
		tempMessages = [];
		pendingMessages = [];
		updateScheduled = false;

		const reader = new Channel<ChatEvent>();

		let id = 0;

		reader.onmessage = ({ event, data }) => {
			if (event === 'message' && data) {
				data.id = id++;

				if (!autoScroll) {
					tempMessages = [...tempMessages, data];
				} else {
					pendingMessages.push(data);

					if (!updateScheduled) {
						updateScheduled = true;

						requestAnimationFrame(() => {
							let combined = [...messages, ...pendingMessages];
							if (combined.length > 300) {
								combined = combined.slice(combined.length - 300);
							}

							messages = combined;
							pendingMessages = [];
							updateScheduled = false;
						});
					}
				}
			}
		};

		command('join_chat', { username, reader });

		return () => {
			emit('leave_chat', username);
		};
	});
</script>

<div class="flex h-full flex-col border-l-2 border-white/20 text-sm select-text">
	<div class="flex">
		<div class="flex-1"></div>

		<button title="Hide chat" class="p-2 hover:bg-neutral-700" onclick={toggleChat}>
			<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 2048 2048"
				><!-- Icon from Fluent UI MDL2 by Microsoft Corporation - https://github.com/microsoft/fluentui/blob/master/packages/react-icons-mdl2/LICENSE --><path
					fill="currentColor"
					d="m903 146l879 878l-879 878l121 121l999-999l-999-999zm-853 0l878 878l-878 878l121 121l999-999L171 25z"
				/></svg
			>
		</button>
	</div>

	<hr class="w-full border-white/20" />

	<div
		data-simplebar
		bind:this={chatContainer}
		class="h-full w-full overflow-y-auto bg-neutral-800"
	>
		{#each messages as message (message.id)}
			<div
				class="px-1 py-1 text-pretty {message.f
					? 'bg-purple-500/20 hover:bg-purple-400/40'
					: 'hover:bg-neutral-800'}"
			>
				<span class="font-bold break-words" style="color: {message.c}"
					>{message.n}<span class="text-white">:</span></span
				>

				{#each message.m as fragment, index (index)}
					{#if fragment.t === 0}
						<span class="break-words">{fragment.c}</span>
					{:else if fragment.t === 1 && fragment.e}
						<img
							loading="lazy"
							class="mx-2 inline-block align-middle"
							src={fragment.e.u}
							alt={fragment.e.n}
							width={fragment.e.w}
							height={fragment.e.h}
							title={fragment.e.n}
						/>
					{:else}
						<!-- svelte-ignore a11y_click_events_have_key_events -->
						<span
							id="url"
							onclick={openURLInBrowser}
							tabindex="-1"
							role="link"
							class="underline-blue-400 mx-2 cursor-pointer break-all text-blue-400 underline hover:text-blue-600"
						>
							{fragment.c}
						</span>
					{/if}
				{/each}
			</div>
		{/each}
	</div>

	{#if !autoScroll}
		<button
			class="absolute right-0 bottom-0 z-50 -translate-x-1/2 transform cursor-pointer rounded-md bg-slate-800/80 p-1 text-center text-white shadow-lg hover:bg-slate-600/90"
			onclick={() => {
				simpleBarInstance.scrollTop = simpleBarInstance.scrollHeight;
			}}
		>
			<span class="shadow-lg">
				{#if tempMessages.length === 0}
					Chat paused
				{:else}
					{tempMessages.length} new messages
				{/if}
			</span>
		</button>
	{/if}
</div>
