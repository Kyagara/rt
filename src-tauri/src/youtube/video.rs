use std::collections::HashMap;

use anyhow::Result;
use futures_util::future;
use log::{error, info};
use rustypipe::model::{
    richtext::ToMarkdown, AudioCodec, AudioStream, ChannelTag, Chapter, Subtitle, VideoCodec,
    VideoDetails, VideoStream,
};
use serde::Serialize;
use sqlx::prelude::FromRow;

use super::main::{RP_CLIENT, USING_BOTGUARD};

const DASH_IDS: [u32; 33] = [
    133, 134, 135, 136, 137, 138, 160, 212, 264, 298, 299, 266, 167, 168, 169, 170, 218, 219, 278,
    242, 243, 244, 245, 246, 247, 248, 271, 272, 302, 303, 308, 313, 315,
];

#[derive(Serialize, FromRow)]
pub struct FeedPageVideo {
    pub id: String,
    pub username: String,
    pub title: String,
    #[serde(rename = "publishedAt")]
    pub published_at: i64,
    #[serde(rename = "viewCount")]
    pub view_count: String,
}

pub async fn fetch_feed_videos(channel_ids: Vec<String>) -> Result<Vec<FeedPageVideo>> {
    let mut videos: HashMap<String, FeedPageVideo> = HashMap::new();

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
            let video = FeedPageVideo {
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

    let videos: Vec<FeedPageVideo> = videos.into_values().collect();

    Ok(videos)
}

#[derive(Serialize)]
pub struct WatchPageVideo {
    id: String,
    metadata: Metadata,
    channel: Channel,
    chapters: Vec<VideoChapter>,
    subtitles: Vec<VideoSubtitle>,
    #[serde(rename = "videoFormats")]
    video_formats: Vec<Format>,
    #[serde(rename = "audioFormats")]
    audio_formats: Vec<Format>,
}

#[derive(Serialize)]
struct Metadata {
    title: String,
    description: String,
    #[serde(rename = "publishedDateTxt")]
    published_date_txt: String,
    #[serde(rename = "viewCount")]
    view_count: u64,
}

#[derive(Serialize)]
struct Channel {
    id: String,
    name: String,
    avatar: String,
}

#[derive(Serialize)]
struct VideoChapter {
    name: String,
    position: u32,
}

#[derive(Serialize)]
pub struct VideoSubtitle {
    url: String,
    lang: String,
    #[serde(rename = "langName")]
    lang_name: String,
    #[serde(rename = "autoGenerated")]
    auto_generated: bool,
}

#[derive(Serialize)]
struct Format {
    src: String,
    #[serde(rename = "type")]
    type_: String,
    codec: String,
    height: u32,
    width: u32,
}

#[tauri::command]
pub async fn fetch_video(video_id: &str) -> Result<WatchPageVideo, String> {
    let client = RP_CLIENT.lock().await;
    let query = client.query();
    let clients = query.player_client_order();

    let details = query.video_details(video_id);
    let mut player = None;

    let using_embed = if *USING_BOTGUARD.lock().await {
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

    let details = results
        .0
        .map_err(|err| format!("Fetching video metadata: {err}"))?;

    let metadata = get_metadata(&details);
    let channel = get_channel(&details.channel);
    let chapters = get_chapters(&details.chapters);

    let (video, audio, subtitles) = if using_embed {
        (Vec::new(), Vec::new(), Vec::new())
    } else {
        let player = results
            .1
            .unwrap()
            .map_err(|err| format!("Fetching player: {err}"))?;

        let subtitles = get_subtitles(&player.subtitles);

        let (video, audio) = get_player_formats(&player.video_only_streams, &player.audio_streams);

        (video, audio, subtitles)
    };

    let watch_page_video = WatchPageVideo {
        id: details.id,
        metadata,
        channel,
        chapters,
        subtitles,
        video_formats: video,
        audio_formats: audio,
    };

    Ok(watch_page_video)
}

fn get_metadata(details: &VideoDetails) -> Metadata {
    Metadata {
        title: details.name.clone(),
        description: details.description.to_markdown(),
        published_date_txt: details
            .publish_date_txt
            .clone()
            .unwrap_or(String::from("N/A")),
        view_count: details.view_count,
    }
}

fn get_channel(channel: &ChannelTag) -> Channel {
    Channel {
        id: channel.id.clone(),
        name: channel.name.clone(),
        avatar: channel
            .avatar
            .first()
            .map(|a| a.url.clone())
            .unwrap_or_default(),
    }
}

fn get_chapters(chapters: &[Chapter]) -> Vec<VideoChapter> {
    chapters
        .iter()
        .map(|c| VideoChapter {
            name: c.name.clone(),
            position: c.position,
        })
        .collect::<Vec<VideoChapter>>()
}

fn get_subtitles(subtitles: &[Subtitle]) -> Vec<VideoSubtitle> {
    subtitles
        .iter()
        .map(|s| VideoSubtitle {
            url: s.url.clone(),
            lang: s.lang.clone(),
            lang_name: s.lang_name.clone(),
            auto_generated: s.auto_generated,
        })
        .collect::<Vec<VideoSubtitle>>()
}

fn get_player_formats(
    video_streams: &[VideoStream],
    audio_streams: &[AudioStream],
) -> (Vec<Format>, Vec<Format>) {
    let video = video_streams
        .iter()
        .filter_map(|v| {
            if !DASH_IDS.contains(&v.itag) {
                return None;
            }

            let codec = match v.codec {
                VideoCodec::Unknown => "Unknown",
                VideoCodec::Mp4v => "mp4v",
                VideoCodec::Avc1 => "avc1",
                VideoCodec::Vp9 => "vp9",
                VideoCodec::Av01 => "av01",
                _ => "Format not supported",
            };

            Some(Format {
                codec: codec.to_string(),
                src: v.url.clone(),
                type_: v.mime.split(';').next().unwrap_or("").trim().to_string(),
                height: v.height,
                width: v.width,
            })
        })
        .collect::<Vec<Format>>();

    let audio = audio_streams
        .iter()
        .map(|a| {
            let codec = match a.codec {
                AudioCodec::Unknown => "Unknown",
                AudioCodec::Mp4a => "mp4a",
                AudioCodec::Opus => "opus",
                AudioCodec::Ac3 => "ac-3",
                AudioCodec::Ec3 => "ec-3",
                _ => "Format not supported",
            };

            Format {
                codec: codec.to_string(),
                src: a.url.clone(),
                type_: a.mime.split(';').next().unwrap_or("").trim().to_string(),
                height: 0,
                width: 0,
            }
        })
        .collect::<Vec<Format>>();

    (video, audio)
}
