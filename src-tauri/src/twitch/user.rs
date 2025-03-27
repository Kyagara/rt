use std::collections::HashMap;

use anyhow::{anyhow, Result};
use log::error;

use crate::{
    user::{Platform, User},
    util,
};

use super::{
    emote::{self, Emote},
    main,
    query::{TurboAndSubUpsellQuery, TurboAndSubUpsellResponse},
};

pub async fn fetch_user(username: &str) -> Result<(User, HashMap<String, Emote>)> {
    let gql = TurboAndSubUpsellQuery::new(username);

    let response: TurboAndSubUpsellResponse = match main::send_query(gql).await {
        Ok(response) => response,
        Err(err) => {
            return Err(anyhow!("Requesting user '{username}': {err}"));
        }
    };

    let Some(user) = response.data.user else {
        return Err(anyhow!("User '{username}' not found"));
    };

    let mut user_emotes = emote::parse_subscription_products(user.subscription_products);

    let user_id = user.id;

    let seventv_emotes = match emote::fetch_7tv_emotes(&user_id).await {
        Ok(emotes) => emotes,
        Err(err) => {
            error!("Fetching 7tv emotes: {err}");
            HashMap::new()
        }
    };

    let bettertv_emotes = match emote::fetch_bettertv_emotes(&user_id).await {
        Ok(emotes) => emotes,
        Err(err) => {
            error!("Fetching bettertv emotes: {err}");
            HashMap::new()
        }
    };

    user_emotes.extend(seventv_emotes);
    user_emotes.extend(bettertv_emotes);

    let avatar = util::download_image(&user.profile_image_url).await?;

    let user = User {
        id: user_id,
        username: username.to_string(),
        avatar,
        platform: Platform::Twitch,
    };

    Ok((user, user_emotes))
}
