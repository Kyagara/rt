<script lang="ts">
	import { onMount } from 'svelte';

	import { getCurrent } from '@tauri-apps/plugin-deep-link';

	import { notify } from '$lib/components/Notification.svelte';

	import { changeView } from '$lib/state/View.svelte';

	const twitchReg = /(?:https?:\/\/)?(?:www\.)?twitch\.tv\/([a-zA-Z0-9_]+)/;
	const youtubeReg =
		/(?:https?:\/\/)?(?:www\.)?(?:youtube\.com\/(?:[^/]+\/.+\/|(?:v|embed|shorts|watch)?\??v=|.*[?&]v=)|youtu\.be\/)([^"&?/\s]{11})/;

	async function handleURL(url: string) {
		if (!url) {
			notify('No URL provided');
			return;
		}

		if (url.startsWith('rt://tw/') || url.startsWith('rt://twitch/')) {
			const username = url.replace(/^rt:\/\/(tw|twitch)\//, '').trim();
			if (!username) {
				notify('Username not found in URL');
				return;
			}

			changeView('streams', true, `/watch?username=${username}`);
			return;
		}

		let matches = url.match(twitchReg);
		if (matches && matches[1]) {
			const username = matches[1];

			changeView('streams', true, `/watch?username=${username}`);
			return;
		}

		if (url.startsWith('rt://yt/') || url.startsWith('rt://youtube/')) {
			const videoId = url.replace(/^rt:\/\/(yt|youtube)\//, '').trim();
			if (!videoId) {
				notify('Video ID not found in URL');
				return;
			}

			changeView('videos', true, `/watch?id=${videoId}`);
			return;
		}

		matches = url.match(youtubeReg);
		if (matches && matches[1]) {
			const videoId = matches[1];

			changeView('videos', true, `/watch?id=${videoId}`);
			return;
		}

		notify('Could not retrieve frontend from URL');
	}

	onMount(async () => {
		try {
			const current = await getCurrent();
			if (current && current[0]) {
				await handleURL(current[0]);
			} else {
				changeView(localStorage.getItem('lastView') || 'videos');
			}
		} catch (err) {
			notify(`Error handling URL: ${err}`);
		}
	});
</script>
