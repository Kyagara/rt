import type { VideoMimeType } from 'vidstack';

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

	type YoutubePlayer = {
		id: string;
		title: string;
		description: string;
		chapters: YouTubeChapter[];
		subtitles: YouTubeSubtitle[];
		published_date_txt: string;
		view_count: number;
		is_live: boolean;
		sources: YouTubeSource[];
		audio: string;
		channel_id: string;
		channel_name: string;
		channel_avatar: string;
	};

	type YouTubeChapter = {
		name: string;
		position: number;
	};

	type YouTubeSubtitle = {
		url: string;
		lang: string;
		lang_name: string;
		auto_generated: boolean;
	};

	type YouTubeSource = {
		url: string;
		format: VideoMimeType;
		height: number;
		width: number;
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
