use anyhow::Result;
use futures_util::future;
use log::error;
use serde::Serialize;
use sqlx::prelude::FromRow;

use super::main::RP_CLIENT;

#[derive(Serialize, FromRow)]
pub struct YouTubeVideo {
    pub id: String,
    pub username: String,
    pub title: String,
    pub published_at: i64,
    pub view_count: String,
}

pub async fn fetch_videos(channel_ids: Vec<String>) -> Result<Vec<YouTubeVideo>> {
    let mut videos = Vec::new();

    let client = RP_CLIENT.lock().await;
    let query = client.query();

    let futures = channel_ids
        .into_iter()
        .map(|channel_id| query.channel_rss(channel_id));

    let results = future::join_all(futures).await;

    for channel in results {
        if channel.is_err() {
            error!("Failed to fetch channel feed: {:?}", channel.err());
            continue;
        }

        let channel = channel.unwrap();

        videos.reserve_exact(channel.videos.len());

        channel.videos.iter().for_each(|video| {
            let video = YouTubeVideo {
                id: video.id.clone(),
                username: channel.name.clone(),
                title: video.name.clone(),
                published_at: video.publish_date.unix_timestamp(),
                view_count: video.view_count.to_string(),
            };

            videos.push(video);
        });
    }

    Ok(videos)
}
