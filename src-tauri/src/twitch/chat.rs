use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::sleep,
    time::Duration,
};

use futures_util::{SinkExt, StreamExt};
use lazy_static::lazy_static;
use log::error;
use regex::Regex;
use serde::Serialize;
use tauri::{
    async_runtime::{self, Mutex},
    ipc::Channel,
    AppHandle, Listener, State,
};
use tokio_tungstenite::tungstenite::Message;

use crate::{twitch::emote, util, AppState};

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
pub async fn join_chat(
    app_handle: AppHandle,
    state: State<'_, Mutex<AppState>>,
    username: &str,
    reader: Channel<ChatEvent>,
) -> Result<(), String> {
    let user_emotes = {
        let state = state.lock().await;
        let emotes_db = state.emotes_db.as_ref().unwrap();

        emote::query_user_emotes(emotes_db, username)
            .await
            .unwrap_or_default()
    };

    let mut ws_stream = match tokio_tungstenite::connect_async(WS_CHAT_URL).await {
        Ok((ws_stream, _)) => ws_stream,
        Err(err) => {
            return Err(format!("Connecting to chat: {err}"));
        }
    };

    if let Err(err) = ws_stream.send("CAP REQ :twitch.tv/tags".into()).await {
        return Err(format!("Sending CAP REQ: {err}"));
    }

    if let Err(err) = ws_stream.send("PASS SCHMOOPIIE".into()).await {
        return Err(format!("Sending PASS: {err}"));
    }

    let random_number = util::random_number(10_000, 99_999);

    if let Err(err) = ws_stream
        .send(format!("NICK justinfan{random_number}").into())
        .await
    {
        return Err(format!("Sending NICK: {err}"));
    }

    if let Err(err) = ws_stream.send(format!("JOIN #{username}").into()).await {
        return Err(format!("Sending JOIN: {err}"));
    }

    let (ws_sink, mut ws_stream) = ws_stream.split();

    let ws_sink = Arc::new(Mutex::new(ws_sink));

    let is_cancelled = Arc::new(AtomicBool::new(false));
    let cancel_flag = Arc::clone(&is_cancelled);

    let listener = app_handle.listen("leave_chat", move |_event| {
        cancel_flag.store(true, Ordering::SeqCst);
    });

    while let Some(Ok(Message::Text(text))) = ws_stream.next().await {
        if is_cancelled.load(Ordering::Relaxed) {
            break;
        }

        // Handle PING/PONG messages
        if text.starts_with(PING) {
            let ws_sink = Arc::clone(&ws_sink);

            if let Err(err) = ws_sink.lock().await.send(Message::text(PONG)).await {
                error!("Sending PONG: {err}");
                continue;
            }

            // Schedule a PING after 60 seconds
            let ws_sink = Arc::clone(&ws_sink);
            async_runtime::spawn(async move {
                sleep(Duration::from_secs(60));

                if let Err(err) = ws_sink.lock().await.send(PING.into()).await {
                    error!("Sending scheduled PING: {err}");
                }
            });

            continue;
        }

        if let Some(caps) = IRC_CHAT_REG.captures(&text) {
            if caps.len() < 5 {
                continue;
            }

            let color = match caps.name("color") {
                Some(color) => color.as_str().to_string(),
                _ => String::new(),
            };

            let display_name = match caps.name("display_name") {
                Some(display_name) => display_name.as_str().to_string(),
                _ => continue,
            };

            let first_msg = match caps.name("first_msg") {
                Some(first_msg) => first_msg.as_str() != "0",
                _ => false,
            };

            let content = match caps.name("message") {
                Some(content) => content.as_str().trim_end(),
                _ => continue,
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
                error!("Sending chat message: {err}");
            }
        }
    }

    app_handle.unlisten(listener);

    Ok(())
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
