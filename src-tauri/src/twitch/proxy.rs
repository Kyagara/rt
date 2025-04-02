use std::collections::HashMap;

use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use log::error;
use tauri::{async_runtime::Mutex, AppHandle, Emitter, EventTarget};

use super::{main::PROXY_HTTP_CLIENT, stream};

lazy_static! {
    static ref STREAM_STATE: Mutex<HashMap<String, StreamState>> = Mutex::new(HashMap::new());
}

#[derive(Clone)]
struct StreamState {
    using_backup: bool,
    main_stream_url: Option<String>,
    backup_stream_url: Option<String>,
}

async fn update_stream_state(window_label: String, new_state: StreamState) {
    let mut stream_state = STREAM_STATE.lock().await;
    stream_state.insert(window_label, new_state);
}

#[tauri::command]
pub async fn proxy_stream(
    app_handle: AppHandle,
    window_label: String,
    username: &str,
    url: &str,
) -> Result<String, String> {
    if url.is_empty() {
        return Err(String::from("No URL provided"));
    }

    let response = match PROXY_HTTP_CLIENT.get(url).send().await {
        Ok(resp) => resp,
        Err(err) => {
            return Err(format!("Proxying request: {err}"));
        }
    };

    let body_bytes = match response.bytes().await {
        Ok(bytes) => bytes,
        Err(err) => {
            return Err(format!("Reading response body: {err}"));
        }
    };

    let (ad_detected, mut playlist, is_master_playlist) =
        process_m3u8(&String::from_utf8_lossy(&body_bytes));

    let mut stream_state = {
        let lock = STREAM_STATE.lock().await;

        let state = lock.get(&window_label).unwrap_or(&StreamState {
            using_backup: false,
            main_stream_url: None,
            backup_stream_url: None,
        });

        state.clone()
    };

    if !is_master_playlist {
        {
            if stream_state.main_stream_url.is_none() {
                stream_state.main_stream_url = Some(url.to_string());
            }
        }

        if ad_detected {
            // On ad detection, if backup isn’t already enabled, switch to backup
            if !stream_state.using_backup {
                if let Err(err) = app_handle.emit_to(
                    EventTarget::WebviewWindow {
                        label: window_label.clone(),
                    },
                    "stream",
                    "backup",
                ) {
                    error!("Emitting change to backup stream event: {err}");
                }

                stream_state.using_backup = true;
            }

            // Use the cached backup stream URL if available, if not, fetch it once
            let backup_url = {
                if let Some(url) = stream_state.backup_stream_url.clone() {
                    url
                } else {
                    let url = fetch_backup_stream_url(username).await.unwrap_or_default();
                    stream_state.backup_stream_url = Some(url.clone());
                    url
                }
            };

            // Fetch an updated backup manifest from the cached backup URL
            match fetch_playlist_text(&backup_url).await {
                Ok(updated_playlist) => playlist = updated_playlist,
                Err(err) => {
                    error!("Fetching updated backup manifest: {err}");
                    playlist.clear();
                }
            }
        } else if stream_state.using_backup {
            // If no ad is detected but we are still in backup, switch back to the main stream
            if let Err(err) = app_handle.emit_to(
                EventTarget::WebviewWindow {
                    label: window_label.clone(),
                },
                "stream",
                "main",
            ) {
                error!("Emitting change to main stream event: {err}");
            }

            match fetch_main_stream(username, &mut stream_state).await {
                Ok(pl) => playlist = pl,
                Err(err) => {
                    error!("Fetching main stream: {err}");
                    playlist.clear();
                }
            }

            stream_state.using_backup = false;
            stream_state.backup_stream_url = None;
        }
    }

    update_stream_state(window_label, stream_state).await;

    Ok(playlist)
}

fn process_m3u8(playlist: &str) -> (bool, String, bool) {
    let mut result_lines = Vec::new();
    let mut ad_detected = false;
    let mut is_master_playlist = false;

    for line in playlist.lines() {
        if line.starts_with("#EXT-X-STREAM-INF") {
            is_master_playlist = true;
        }

        if line.contains("stitched-ad") {
            ad_detected = true;
        }

        result_lines.push(line.to_string());
    }

    (ad_detected, result_lines.join("\n"), is_master_playlist)
}

async fn fetch_playlist_text(url: &str) -> Result<String> {
    let response = PROXY_HTTP_CLIENT
        .get(url)
        .send()
        .await
        .map_err(|err| anyhow!("Requesting playlist: {err}"))?;

    response
        .text()
        .await
        .map_err(|err| anyhow!("Reading playlist text: {err}"))
}

async fn fetch_main_stream(username: &str, stream_state: &mut StreamState) -> Result<String> {
    let Some(main_url) = stream_state.main_stream_url.clone() else {
        error!("Main stream URL not found. Falling back to backup stream.");

        let backup_url = fetch_backup_stream_url(username).await?;

        return fetch_playlist_text(&backup_url).await;
    };

    let body = fetch_playlist_text(&main_url).await?;
    let (ad_detected, processed_playlist, _) = process_m3u8(&body);

    if ad_detected {
        stream_state.using_backup = true;
        return Ok("#EXTM3U\n#EXT-X-ENDLIST\n".to_string());
    }

    stream_state.using_backup = false;
    Ok(processed_playlist)
}

async fn fetch_backup_stream_url(username: &str) -> Result<String> {
    let url = stream::fetch_stream_playback(username, true)
        .await
        .map_err(|err| anyhow!("Fetching backup stream: {err}"))?;

    let body = fetch_playlist_text(&url).await?;

    let variant_playlist_url = body
        .lines()
        .nth(4)
        .ok_or_else(|| anyhow!("Backup master playlist is malformed."))?
        .to_string();

    Ok(variant_playlist_url)
}
