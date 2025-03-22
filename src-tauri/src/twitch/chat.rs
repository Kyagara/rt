use std::{collections::HashMap, sync::Arc, thread::sleep, time::Duration};

use futures_util::{SinkExt, StreamExt};
use lazy_static::lazy_static;
use log::error;
use regex::Regex;
use serde::Serialize;
use tauri::{
    async_runtime::{self, Mutex},
    ipc::Channel,
};
use tokio_tungstenite::tungstenite::Message;

use crate::{twitch::emote, utils};

use super::emote::Emote;

const WS_CHAT_URL: &str = "wss://irc-ws.chat.twitch.tv";
const PING: &str = "PING";
const PONG: &str = "PONG";

lazy_static! {
    static ref IRC_CHAT_REG: Regex = Regex::new(
         r"(?m)^@.*?color=(?P<color>[^;]*).*?display-name=(?P<display_name>[^;]*).*?first-msg=(?P<first_msg>[^;]*).*?PRIVMSG\s+\S+\s+:(?P<message>.*)$"
    ).unwrap();

    static ref URL_REG: Regex = Regex::new(
        r"(?m)(https?:\/\/)?(www\.)?([a-zA-Z0-9-]{1,256})\.[a-zA-Z0-9]{2,}(\/[^\s]*)?"
    ).unwrap();
}

#[derive(Serialize, Clone)]
pub struct ChatMessage {
    #[serde(rename = "c")]
    color: String,
    #[serde(rename = "n")]
    name: String,
    #[serde(rename = "f")]
    first_msg: bool,
    #[serde(rename = "m")]
    fragments: Vec<Fragment>,
}

#[derive(Serialize, Clone)]
struct Fragment {
    #[serde(rename = "t")]
    r#type: u8,
    #[serde(rename = "c")]
    content: String,
    #[serde(rename = "e", skip_serializing_if = "Option::is_none")]
    emote: Option<Emote>,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase", tag = "event", content = "data")]
pub enum ChatEvent {
    #[serde(rename_all = "camelCase")]
    Message(ChatMessage),
}

#[tauri::command]
pub async fn join_chat(username: String, reader: Channel<ChatEvent>) {
    let user_emotes = emote::query_user_emotes(&username)
        .await
        .unwrap_or_default();

    let mut ws_stream = match tokio_tungstenite::connect_async(WS_CHAT_URL).await {
        Ok((ws_stream, _)) => ws_stream,
        Err(err) => {
            error!("Failed to connect to chat: {err}");
            return;
        }
    };

    if let Err(err) = ws_stream.send("CAP REQ :twitch.tv/tags".into()).await {
        error!("Failed to send CAP REQ: {err}");
        return;
    }

    if let Err(err) = ws_stream.send("PASS SCHMOOPIIE".into()).await {
        error!("Failed to send PASS: {err}");
        return;
    }

    let random_number = utils::random_number(10_000, 99_999);

    if let Err(err) = ws_stream
        .send(format!("NICK justinfan{random_number}").into())
        .await
    {
        error!("Failed to send NICK: {err}");
        return;
    }

    if let Err(err) = ws_stream.send(format!("JOIN #{username}").into()).await {
        error!("Failed to send JOIN: {err}");
        return;
    }

    let (ws_sink, mut ws_stream) = ws_stream.split();

    let ws_sink = Arc::new(Mutex::new(ws_sink));

    while let Some(Ok(Message::Text(text))) = ws_stream.next().await {
        if text.starts_with(PING) {
            let ws_sink = Arc::clone(&ws_sink);

            if let Err(err) = ws_sink.lock().await.send(Message::text(PONG)).await {
                error!("Failed to send PONG: {err}");
                continue;
            }

            // Ping the server after 60 seconds
            let ws_sink = Arc::clone(&ws_sink);

            async_runtime::spawn(async move {
                sleep(Duration::from_secs(60));

                if let Err(err) = ws_sink.lock().await.send(PING.into()).await {
                    error!("Failed to send scheduled PING: {err}");
                }
            });

            continue;
        }

        if let Some(caps) = IRC_CHAT_REG.captures(&text) {
            if caps.len() < 5 {
                continue;
            }

            let color = if let Some(color) = caps.name("color") {
                color.as_str().to_string()
            } else {
                String::new()
            };

            let display_name = if let Some(display_name) = caps.name("display_name") {
                display_name.as_str().to_string()
            } else {
                continue;
            };

            let first_msg = if let Some(first_msg) = caps.name("first_msg") {
                first_msg.as_str() != "0"
            } else {
                false
            };

            let content = if let Some(content) = caps.name("message") {
                content.as_str().trim_end()
            } else {
                continue;
            };

            let fragments = parse_chat_fragments(content, &user_emotes);
            if fragments.is_empty() {
                continue;
            }

            let chat_message = ChatMessage {
                color,
                name: display_name,
                first_msg,
                fragments,
            };

            if let Err(err) = reader.send(ChatEvent::Message(chat_message)) {
                error!("Failed to send chat message: {err}");
            }
        }
    }
}

fn parse_chat_fragments(
    message_content: &str,
    user_emotes: &HashMap<String, Emote>,
) -> Vec<Fragment> {
    let mut fragments = Vec::new();

    // This initializer value was revealed to me in a dream
    let mut last_type = 10;

    for token in message_content.split_whitespace() {
        let current_type;

        if URL_REG.is_match(token) {
            current_type = 2;
        } else if user_emotes.contains_key(token) {
            current_type = 1;
        } else {
            current_type = 0;
        }

        if current_type != last_type {
            let emote = if current_type == 1 {
                user_emotes.get(token).cloned()
            } else {
                None
            };

            fragments.push(Fragment {
                r#type: current_type,
                content: token.to_string(),
                emote,
            });

            last_type = current_type;
            continue;
        }

        if current_type == 0 {
            // Append to last fragment with an whitespace
            fragments
                .last_mut()
                .unwrap()
                .content
                .push_str(format!(" {token}").as_str());
        }
    }

    fragments
}
