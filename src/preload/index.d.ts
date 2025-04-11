declare global {
	interface Window {
		main: {
			newWindow: (route: string) => void
		}

		titlebar: {
			onResized: (callback: (isMaximized: boolean) => void) => void
			onMaximized: (callback: (maximized: boolean) => void) => void
			minimize: () => void
			maximize: () => void
			close: () => void
		}

		user: {
			add: (platform: Platform, username: string, id?: string) => Promise<User | null>
			get: (platform: Platform, username?: string, id?: string) => Promise<User | null>
			list: (platform?: Platform) => Promise<User[]>
			remove: (platform: Platform, username: string) => Promise<string | null>
		}

		feed: {
			get: (platform: Platform, lastPublishedAt?: number) => Promise<Feed>
			refresh: (platform: Platform) => Promise<Feed>
		}

		stream: {
			get: (username: string, backup: boolean) => Promise<string>
			emotes: (username: string) => Promise<Record<string, Emote>>
		}

		video: {
			get: (videoID: string, retrievePlayer: boolean) => Promise<WatchPageVideo>
			import: () => Promise<number>
		}
	}
}

export {}
