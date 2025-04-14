<script lang="ts">
	import 'vidstack/bundle'

	type VideoWatchPageProps = {
		player: WatchPageVideo
		usingEmbed: boolean
		autoPlay: boolean
	}

	let { player, usingEmbed, autoPlay }: VideoWatchPageProps = $props()
</script>

{#if usingEmbed}
	<media-player
		storage="player-settings"
		src={`https://youtu.be/${player.id}`}
		{autoPlay}
		streamType="on-demand"
		style="--plyr-border-radius: 0px;"
	>
		<media-provider></media-provider>

		<media-plyr-layout displayDuration={true}></media-plyr-layout>
	</media-player>
{:else}
	<media-player {autoPlay} storage="player-settings" style="--plyr-border-radius: 0px;">
		<media-provider>
			{#if player.isLive}
				{#if player.live.hls}
					<source src={player.live.hls} type="application/x-mpegURL" />
				{:else}
					<source
						src={`data:application/dash+xml;charset=utf-8;base64,${btoa(player.live.dash)}`}
						type="application/dash+xml"
					/>
				{/if}
			{:else}
				<source
					src={`data:application/dash+xml;charset=utf-8;base64,${btoa(player.dash)}`}
					type="application/dash+xml"
				/>
			{/if}
		</media-provider>

		<media-plyr-layout displayDuration={true}></media-plyr-layout>
	</media-player>
{/if}
