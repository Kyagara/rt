use anyhow::Result;
use futures_util::future;
use log::error;

use crate::{
    user::{Platform, User},
    util,
};

use super::main::RP_CLIENT;

pub async fn fetch_channel_by_name(channel_name: &str) -> Result<User> {
    let client = RP_CLIENT.lock().await;

    let url_target = client.query().resolve_string(channel_name, false).await?;
    let url = url_target.to_url();
    let channel_id = url.split('/').last().unwrap();

    let channel = client.query().channel_videos(channel_id).await?;

    let avatar = match channel.avatar.first() {
        Some(avatar) => util::download_image(&avatar.url).await,
        None => Vec::new(),
    };

    let user = User {
        id: channel.id,
        username: channel.name,
        avatar,
        platform: Platform::YouTube,
    };

    Ok(user)
}

pub async fn fetch_channels_by_id(channel_ids: Vec<String>) -> Result<Vec<User>> {
    let client = RP_CLIENT.lock().await;
    let query = client.query();

    let futures = channel_ids.into_iter().map(|id| query.channel_videos(id));

    let results = future::join_all(futures).await;

    let mut users = Vec::with_capacity(results.len());

    for result in results {
        if let Err(err) = result {
            error!("Fetching channel by id: {err}");
            continue;
        }

        let channel = result.unwrap();

        let avatar = match channel.avatar.first() {
            Some(avatar) => util::download_image(&avatar.url).await,
            None => Vec::new(),
        };

        let user = User {
            id: channel.id,
            username: channel.name,
            avatar,
            platform: Platform::YouTube,
        };

        users.push(user);
    }

    Ok(users)
}
