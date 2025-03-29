use std::collections::HashMap;

use anyhow::Result;
use futures_util::future;
use log::{error, info};
use rustypipe::{model::richtext::ToMarkdown, param::StreamFilter};
use serde::Serialize;
use sqlx::prelude::FromRow;

use super::main::RP_CLIENT;

#[derive(Serialize)]
pub struct YouTubePlayer {
    title: String,
    description: String,
    chapters: Vec<YouTubeChapter>,
    subtitles: Vec<YouTubeSubtitle>,
    published_date_txt: String,
    view_count: u64,
    #[serde(rename = "isLive")]
    is_live: bool,
    sources: Vec<YouTubeSource>,
    audio: String,
    channel_id: String,
    channel_name: String,
    channel_avatar: String,
}

#[derive(Serialize)]
pub struct YouTubeChapter {
    name: String,
    position: u32,
}

#[derive(Serialize)]
pub struct YouTubeSubtitle {
    url: String,
    lang: String,
    lang_name: String,
    auto_generated: bool,
}

#[derive(Serialize)]
pub struct YouTubeSource {
    url: String,
    format: String,
    height: u32,
    width: u32,
}

#[tauri::command]
pub async fn fetch_player(video_id: &str) -> Result<YouTubePlayer, String> {
    let client = RP_CLIENT.lock().await;
    let query = client.query();
    let clients = query.player_client_order();

    let details = query.video_details(video_id);
    let mut player = None;

    let using_embed = if client.version_botguard().await.is_some() {
        info!("Using rustypipe-botguard");

        player = Some(query.player_from_clients(video_id, clients));
        false
    } else {
        info!("Using embedded player");
        true
    };

    let results = if using_embed {
        (details.await, None)
    } else {
        let results = future::join(details, player.unwrap()).await;
        (results.0, Some(results.1))
    };

    let metadata = results
        .0
        .map_err(|err| format!("Fetching video metadata: {err}"))?;

    let avatar = metadata
        .channel
        .avatar
        .first()
        .map(|a| a.url.clone())
        .unwrap_or_default();

    let chapters = metadata
        .chapters
        .into_iter()
        .map(|c| YouTubeChapter {
            name: c.name,
            position: c.position,
        })
        .collect::<Vec<YouTubeChapter>>();

    let mut subtitles = Vec::new();

    let mut sources = Vec::new();
    let mut audio = String::new();

    if !using_embed {
        let player = results
            .1
            .unwrap()
            .map_err(|err| format!("Fetching player: {err}"))?;

        subtitles = player
            .subtitles
            .iter()
            .map(|s| YouTubeSubtitle {
                url: s.url.clone(),
                lang: s.lang.clone(),
                lang_name: s.lang_name.clone(),
                auto_generated: s.auto_generated,
            })
            .collect::<Vec<YouTubeSubtitle>>();

        sources = player
            .video_only_streams
            .iter()
            .map(|v| YouTubeSource {
                url: v.url.clone(),
                format: v.mime.split(';').next().unwrap_or("").trim().to_string(),
                height: v.height,
                width: v.width,
            })
            .collect::<Vec<YouTubeSource>>();

        if let Some(stream) = player.select_audio_stream(&StreamFilter::default()) {
            audio.clone_from(&stream.url);
        }
    }

    let player = YouTubePlayer {
        title: metadata.name,
        description: metadata.description.to_markdown(),
        chapters,
        subtitles,
        published_date_txt: metadata.publish_date_txt.unwrap_or("N/A".to_owned()),
        view_count: metadata.view_count,
        is_live: metadata.is_live,
        sources,
        audio,
        channel_id: metadata.channel.id,
        channel_name: metadata.channel.name,
        channel_avatar: avatar,
    };

    Ok(player)
}

#[derive(Serialize, FromRow)]
pub struct YouTubeVideo {
    pub id: String,
    pub username: String,
    pub title: String,
    pub published_at: i64,
    pub view_count: String,
}

pub async fn fetch_videos(channel_ids: Vec<String>) -> Result<Vec<YouTubeVideo>> {
    let mut videos: HashMap<String, YouTubeVideo> = HashMap::new();

    let client = RP_CLIENT.lock().await;
    let query = client.query();

    let futures = channel_ids
        .into_iter()
        .map(|channel_id| query.channel_rss(channel_id));

    let results = future::join_all(futures).await;

    for result in results {
        if let Err(err) = result {
            error!("Fetching channel feed: {err}");
            continue;
        }

        let channel = result.unwrap();

        videos.reserve(channel.videos.len());

        for rss_video in channel.videos {
            let video = YouTubeVideo {
                id: rss_video.id.clone(),
                username: channel.name.clone(),
                title: rss_video.name.clone(),
                published_at: rss_video.publish_date.unix_timestamp(),
                view_count: rss_video.view_count.to_string(),
            };

            if let Some(existing) = videos.get_mut(&video.title) {
                let existing_view_count = existing.view_count.parse::<u64>().unwrap_or(0);

                if rss_video.view_count > existing_view_count {
                    *existing = video;
                }
            } else {
                videos.insert(video.title.clone(), video);
            }
        }
    }

    let videos: Vec<YouTubeVideo> = videos.into_values().collect();

    Ok(videos)
}
