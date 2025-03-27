declare global {
	type User = {
		id: string;
		username: string;
		platform: Platform;
		avatar: number[];
	};

	type Feed = {
		twitch: LiveNow[] | null;
		youtube: YouTubeVideo[] | null;
	};

	type LiveNow = {
		username: string;
		started_at: string;
	};

	type YouTubeVideo = {
		id: string;
		username: string;
		title: string;
		published_at: number;
		view_count: string;
	};

	type ChatEvent = {
		event: 'message';
		data: ChatMessage;
	};

	type ChatMessage = {
		id: number;
		// Color
		c: string;
		// First message, not used
		f: boolean;
		// Name
		n: string;
		// Fragments that make up the message
		m: MessageFragment[];
	};

	type MessageFragment = {
		// Type, 0 = text, 1 = emote, 2 = url
		t: number;
		// Content
		c: string;
		// Emote
		e: Emote;
	};

	type Emote = {
		// Name
		n: string;
		// URL
		u: string;
		// Width
		w: number;
		// Height
		h: number;
	};
}

export {};
