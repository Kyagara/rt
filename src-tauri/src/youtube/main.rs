use std::path::Path;

use std::time::Duration;
use std::{fs::File, io::Read};

use anyhow::Result;
use lazy_static::lazy_static;
use rustypipe::client::RustyPipe;
use sqlx::Row;
use tauri::async_runtime::Mutex;
use tauri::State;
use tauri_plugin_http::reqwest::Client;

use crate::AppState;

use super::channel;

lazy_static! {
    pub static ref RP_CLIENT: Mutex<RustyPipe> = Mutex::new(
        RustyPipe::builder()
            .unauthenticated()
            .no_reporter()
            .no_storage()
            .no_botguard()
            .build()
            .unwrap()
    );
    pub static ref USING_BOTGUARD: Mutex<bool> = Mutex::new(false);
}

pub async fn build_client(storage_dir: &Path) -> Result<()> {
    let mut client = RP_CLIENT.lock().await;

    let http_client = Client::builder()
        .use_rustls_tls()
        .https_only(true)
        .tcp_keepalive(Duration::from_secs(10))
        .http2_prior_knowledge();

    *client = RustyPipe::builder()
        .unauthenticated()
        .storage_dir(storage_dir)
        .build_with_client(http_client)?;

    let version = client.version_botguard().await;
    *USING_BOTGUARD.lock().await = version.is_some();

    Ok(())
}

#[tauri::command]
pub async fn import_subscriptions(
    state: State<'_, Mutex<AppState>>,
    subscriptions_file_path: &str,
) -> Result<i64, String> {
    let mut file = match File::open(subscriptions_file_path) {
        Ok(file) => file,
        Err(err) => return Err(format!("Opening subscriptions file: {err}")),
    };

    let mut contents = String::new();
    if let Err(err) = file.read_to_string(&mut contents) {
        return Err(format!("Reading subscriptions file: {err}"));
    }

    let mut lines = contents.lines();
    lines.next();

    let mut imported_channel_ids = Vec::new();

    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let mut parts = line.split(',');
        let Some(channel_id) = parts.next() else {
            continue;
        };

        imported_channel_ids.push(channel_id.to_string());
    }

    let state = state.lock().await;
    let users_db = state.users_db.as_ref().unwrap();

    let query = "SELECT id from youtube";

    let rows = sqlx::query(query)
        .fetch_all(users_db)
        .await
        .map_err(|err| format!("Querying channel ids: {err}"))?;

    let mut saved_channel_ids = Vec::with_capacity(rows.len());

    for row in rows {
        let channel_id: String = row.get::<String, _>(0);
        saved_channel_ids.push(channel_id);
    }

    imported_channel_ids.retain(|item| !saved_channel_ids.contains(item));

    if imported_channel_ids.is_empty() {
        return Ok(0);
    }

    let new_channels = match channel::fetch_channels_by_id(imported_channel_ids.clone()).await {
        Ok(channels) => channels,
        Err(err) => {
            return Err(format!("Fetching channels: {err}"));
        }
    };

    if new_channels.is_empty() {
        return Ok(0);
    }

    let mut tx = users_db
        .begin()
        .await
        .map_err(|err| format!("Beginning transaction: {err}"))?;

    let placeholders = vec!["(?, ?, ?)"; new_channels.len()].join(", ");

    let sql = format!("INSERT OR IGNORE INTO youtube (id, username, avatar) VALUES {placeholders}");

    let mut query = sqlx::query(&sql);

    for channel in &new_channels {
        query = query
            .bind(&channel.id)
            .bind(&channel.username)
            .bind(&channel.avatar);
    }

    if let Err(err) = query.execute(&mut *tx).await {
        return Err(format!("Executing insert query: {err}"));
    }

    if let Err(err) = tx.commit().await {
        return Err(format!("Committing transaction: {err}"));
    }

    Ok(new_channels.len().try_into().unwrap())
}
