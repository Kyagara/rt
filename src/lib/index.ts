import { invoke } from '@tauri-apps/api/core';
import { error as logError } from '@tauri-apps/plugin-log';

import { notify } from './components/Notification.svelte';

export enum Platform {
	Twitch = 'twitch',
	YouTube = 'youtube'
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export async function command<T>(command: string, data?: any): Promise<T | null> {
	return await invoke<T>(command, data)
		.then((res) => {
			return res;
		})
		.catch((err) => {
			notify(`Error executing '${command}' command`);
			logError(`Invoking '${command}'`, err);
			return null;
		});
}

export function getAvatarUrl(avatar: number[]) {
	const byteArray = new Uint8Array(avatar);
	const blob = new Blob([byteArray], { type: 'image/png' });
	return URL.createObjectURL(blob);
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

function plural(number: number) {
	if (number > 1) {
		return 's';
	}

	return '';
}
