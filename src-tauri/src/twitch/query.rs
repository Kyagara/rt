use serde::{Deserialize, Serialize};

const TURBO_AND_SUB_UPSELL_QUERY_HASH: &str =
    "5dbca380e47e37808c89479f51f789990ec653428a01b76c649ebe01afb3aa7e";

const USE_LIVE_QUERY_HASH: &str =
    "639d5f11bfb8bf3053b424d9ef650d04c4ebb7d94711d644afb08fe9a0fad5d9";

#[derive(Serialize)]
pub struct TurboAndSubUpsellQuery {
    #[serde(rename = "operationName")]
    operation_name: String,
    variables: ChannelLoginVariable,
    extensions: QueryExtensions,
}

impl TurboAndSubUpsellQuery {
    pub fn new(channel_login: &str) -> Self {
        Self {
            operation_name: String::from("TurboAndSubUpsell"),
            variables: ChannelLoginVariable {
                channel_login: channel_login.to_string(),
            },
            extensions: QueryExtensions::new(TURBO_AND_SUB_UPSELL_QUERY_HASH),
        }
    }
}

#[derive(Deserialize)]
pub struct TurboAndSubUpsellResponse {
    pub data: TurboAndSubUpsellData,
}

#[derive(Deserialize)]
pub struct TurboAndSubUpsellData {
    pub user: Option<TurboAndSubUpsellUser>,
}

#[derive(Deserialize)]
pub struct TurboAndSubUpsellUser {
    pub id: String,
    #[serde(rename = "profileImageURL")]
    pub profile_image_url: String,
    #[serde(rename = "subscriptionProducts")]
    pub subscription_products: Vec<TurboAndSubUpsellSubscriptionProduct>,
}

#[derive(Deserialize)]
pub struct TurboAndSubUpsellSubscriptionProduct {
    pub emotes: Vec<TurboAndSubUpsellEmote>,
}

#[derive(Deserialize)]
pub struct TurboAndSubUpsellEmote {
    pub id: String,
    pub token: String,
}

/// For fetching the if the user is streaming
///
/// I don't plan on querying the stream info when refreshing users, so this query is really good for this
#[derive(Serialize)]
pub struct UseLiveQuery {
    #[serde(rename = "operationName")]
    operation_name: String,
    variables: ChannelLoginVariable,
    extensions: QueryExtensions,
}

impl UseLiveQuery {
    pub fn new(channel_login: &str) -> Self {
        Self {
            operation_name: String::from("UseLive"),
            variables: ChannelLoginVariable {
                channel_login: channel_login.to_string(),
            },
            extensions: QueryExtensions::new(USE_LIVE_QUERY_HASH),
        }
    }
}

#[derive(Deserialize)]
pub struct UseLiveResponse {
    pub data: UseLiveData,
}

#[derive(Deserialize)]
pub struct UseLiveData {
    pub user: UseLiveUser,
}

#[derive(Deserialize)]
pub struct UseLiveUser {
    pub login: String,
    // If none, the user is not streaming
    pub stream: Option<UseLiveStream>,
}

#[derive(Deserialize)]
pub struct UseLiveStream {
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

/// For fetching the stream playback access token
#[derive(Serialize)]
pub struct PlaybackAccessTokenQuery {
    #[serde(rename = "operationName")]
    operation_name: String,
    query: String,
    variables: PlaybackAccessTokenQueryVariables,
}

impl PlaybackAccessTokenQuery {
    pub fn new(login: &str, backup: bool) -> Self {
        let player_type = if backup { "autoplay" } else { "site" };
        let platform = if backup { "ios" } else { "web" };

        let query = "query PlaybackAccessToken_Template($login: String!, $isLive: Boolean!, $vodID: ID!, $isVod: Boolean!, $playerType: String!, $platform: String!) {  streamPlaybackAccessToken(channelName: $login, params: {platform: $platform, playerBackend: \"mediaplayer\", playerType: $playerType}) @include(if: $isLive) {    value    signature   authorization { isForbidden forbiddenReasonCode }   __typename  }  videoPlaybackAccessToken(id: $vodID, params: {platform: $platform, playerBackend: \"mediaplayer\", playerType: $playerType}) @include(if: $isVod) {    value    signature   __typename  }}";

        Self {
            operation_name: String::from("PlaybackAccessToken_Template"),
            query: query.to_string(),
            variables: PlaybackAccessTokenQueryVariables {
                login: login.to_string(),
                is_live: true,
                is_vod: false,
                vod_id: String::new(),
                player_type: player_type.to_string(),
                platform: platform.to_string(),
            },
        }
    }
}

#[derive(Serialize)]
pub struct PlaybackAccessTokenQueryVariables {
    login: String,
    #[serde(rename = "isLive")]
    is_live: bool,
    #[serde(rename = "isVod")]
    is_vod: bool,
    #[serde(rename = "vodID")]
    vod_id: String,
    #[serde(rename = "playerType")]
    player_type: String,
    platform: String,
}

#[derive(Deserialize)]
pub struct PlaybackAccessTokenResponse {
    pub data: PlaybackAccessTokenData,
}

#[derive(Deserialize)]
pub struct PlaybackAccessTokenData {
    #[serde(rename = "streamPlaybackAccessToken")]
    pub stream_playback_access_token: PlaybackAccessToken,
}

#[derive(Deserialize)]
pub struct PlaybackAccessToken {
    pub value: String,
    pub signature: String,
}

// Some queries have this variable

#[derive(Serialize, Deserialize)]
pub struct ChannelLoginVariable {
    #[serde(rename = "channelLogin")]
    channel_login: String,
}

// Every persistent query has these fields

#[derive(Serialize)]
pub struct QueryExtensions {
    #[serde(rename = "persistedQuery")]
    persisted_query: PersistedQuery,
}

impl QueryExtensions {
    pub fn new(hash: &str) -> Self {
        Self {
            persisted_query: PersistedQuery {
                version: 1,
                sha256_hash: hash.to_string(),
            },
        }
    }
}

#[derive(Serialize)]
pub struct PersistedQuery {
    version: u64,
    #[serde(rename = "sha256Hash")]
    sha256_hash: String,
}
