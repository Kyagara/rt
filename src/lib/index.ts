import { invoke } from '@tauri-apps/api/core';
import { error as logError } from '@tauri-apps/plugin-log';
import { openUrl } from '@tauri-apps/plugin-opener';

import { notify } from './components/Notification.svelte';

export enum Platform {
	Twitch = 'twitch',
	YouTube = 'youtube'
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export async function command<T>(command: string, data?: any): Promise<T | null> {
	try {
		return await invoke<T>(command, data);
	} catch (err) {
		notify(`Error executing '${command}' command`);
		logError(`Invoking '${command}': ${err}`);
		throw null;
	}
}

const ytAvatarCache = new Map();
const twAvatarCache = new Map();

export function getAvatarUrl(platform: Platform, username: string, avatar: number[]) {
	const cache = platform === Platform.Twitch ? twAvatarCache : ytAvatarCache;
	if (cache.has(username)) {
		return cache.get(username);
	}

	const byteArray = new Uint8Array(avatar);
	const blob = new Blob([byteArray], { type: 'image/png' });
	const url = URL.createObjectURL(blob);

	cache.set(username, url);
	return url;
}

export function timeAgo(timestamp: number) {
	const now = Math.floor(Date.now() / 1000);
	const secondsAgo = now - timestamp;

	if (secondsAgo < 60) return `${secondsAgo} second${plural(secondsAgo)} ago`;
	const minutesAgo = Math.floor(secondsAgo / 60);

	if (minutesAgo < 60) return `${minutesAgo} minute${plural(minutesAgo)} ago`;
	const hoursAgo = Math.floor(minutesAgo / 60);

	if (hoursAgo < 24) return `${hoursAgo} hour${plural(hoursAgo)} ago`;

	const daysAgo = Math.floor(hoursAgo / 24);
	if (daysAgo < 30) return `${daysAgo} day${plural(daysAgo)} ago`;

	const monthsAgo = Math.floor(daysAgo / 30);
	if (monthsAgo < 12) return `${monthsAgo} month${plural(monthsAgo)} ago`;

	const yearsAgo = Math.floor(monthsAgo / 12);
	return `${yearsAgo} year${plural(yearsAgo)} ago`;
}

export function streamingFor(startedAt: string) {
	const diff = new Date().getTime() - new Date(startedAt).getTime();
	const totalSeconds = Math.floor(diff / 1000);
	const hours = Math.floor(totalSeconds / 3600);
	const minutes = Math.floor((totalSeconds % 3600) / 60);
	const seconds = totalSeconds % 60;

	const formattedMinutes = minutes.toString().padStart(2, '0');
	const formattedSeconds = seconds.toString().padStart(2, '0');

	return `${hours}:${formattedMinutes}:${formattedSeconds}`;
}

export async function openURLInBrowser(event: MouseEvent) {
	const target = event.target as HTMLElement;
	if (target.id === 'url') {
		let url = target.innerText;
		if (!url.startsWith('http') && !url.startsWith('https')) {
			url = `https://${url}`;
		}

		await openUrl(url);
	}
}

function plural(number: number) {
	if (number > 1) {
		return 's';
	}

	return '';
}
