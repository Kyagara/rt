use std::path::Path;

use std::{fs::File, io::Read};

use anyhow::Result;
use lazy_static::lazy_static;
use log::error;
use rustypipe::client::RustyPipe;
use sqlx::Row;
use tauri::async_runtime::Mutex;
use tauri::State;

use crate::AppState;

use super::channel;

lazy_static! {
    pub static ref RP_CLIENT: Mutex<RustyPipe> = Mutex::new(
        RustyPipe::builder()
            .no_storage()
            .no_botguard()
            .build()
            .unwrap()
    );
}

pub async fn build_client(storage_dir: &Path) -> Result<()> {
    let mut client = RP_CLIENT.lock().await;

    *client = RustyPipe::builder()
        .unauthenticated()
        .storage_dir(storage_dir)
        .no_botguard()
        .build()?;

    Ok(())
}

#[tauri::command]
pub async fn import_subscriptions(
    state: State<'_, Mutex<AppState>>,
    subscriptions_file_path: &str,
) -> Result<i64, String> {
    let mut file = match File::open(subscriptions_file_path) {
        Ok(file) => file,
        Err(err) => return Err(format!("Failed to open subscriptions file: {err}")),
    };

    let mut contents = String::new();
    if let Err(err) = file.read_to_string(&mut contents) {
        return Err(format!("Failed to read subscriptions file: {err}"));
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
        .map_err(|err| format!("Failed to query channel ids: {err}"))?;

    let mut saved_channel_ids = Vec::with_capacity(rows.len());

    for row in rows {
        let channel_id: String = row.try_get("id").map_err(|e| e.to_string())?;
        saved_channel_ids.push(channel_id);
    }

    imported_channel_ids.retain(|item| !saved_channel_ids.contains(item));

    let mut channels_imported = 0;

    for channel_id in imported_channel_ids {
        match channel::fetch_channel_by_id(&channel_id).await {
            Ok(user) => {
                channels_imported += 1;

                let query = "INSERT INTO youtube (id, username, avatar) VALUES (?, ?, ?) ON CONFLICT (username) DO UPDATE SET avatar = ?";

                if let Err(err) = sqlx::query(query)
                    .bind(&user.id)
                    .bind(&user.username)
                    .bind(&user.avatar)
                    .bind(&user.avatar)
                    .execute(users_db)
                    .await
                {
                    error!("Failed to save channel '{channel_id}': {err}");
                }
            }
            Err(err) => {
                error!("Failed to import channel '{channel_id}': {err}");
            }
        }
    }

    Ok(channels_imported)
}
