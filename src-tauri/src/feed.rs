use anyhow::Result;
use log::error;
use serde::Serialize;
use sqlx::Row;
use tauri::{async_runtime::Mutex, AppHandle, Emitter, State};

use crate::{
    twitch::{self, stream::LiveNow},
    user::Platform,
    youtube::{self, video::YouTubeVideo},
    AppState,
};

#[derive(Serialize)]
pub struct Feed {
    twitch: Option<Vec<LiveNow>>,
    youtube: Option<Vec<YouTubeVideo>>,
}

#[tauri::command]
pub async fn get_feed(
    state: State<'_, Mutex<AppState>>,
    platform: Platform,
    last_published_at: Option<i64>,
) -> Result<Feed, String> {
    let state = state.lock().await;
    let feeds_db = state.feeds_db.as_ref().unwrap();

    if platform == Platform::Twitch {
        let query = "SELECT username, started_at FROM twitch";

        let rows = match sqlx::query(query).fetch_all(feeds_db).await {
            Ok(rows) => rows,
            Err(err) => {
                return Err(format!("Querying feed: {err}"));
            }
        };

        let mut feed: Vec<LiveNow> = Vec::new();

        for row in rows {
            let live_now = LiveNow {
                username: row.get::<String, _>(0),
                started_at: row.get::<String, _>(1),
            };

            feed.push(live_now);
        }

        return Ok(Feed {
            twitch: Some(feed),
            youtube: None,
        });
    }

    if platform == Platform::YouTube {
        let query = if last_published_at.is_some() {
            "SELECT id, username, title, published_at, view_count FROM youtube WHERE published_at < ? ORDER BY published_at DESC LIMIT 50"
        } else {
            "SELECT id, username, title, published_at, view_count FROM youtube ORDER BY published_at DESC LIMIT 50"
        };

        let mut query_builder = sqlx::query_as::<_, YouTubeVideo>(query);

        if let Some(last) = last_published_at {
            query_builder = query_builder.bind(last);
        }

        let feed = query_builder
            .fetch_all(feeds_db)
            .await
            .map_err(|err| format!("Querying feed: {err}"))?;

        return Ok(Feed {
            youtube: Some(feed),
            twitch: None,
        });
    }

    Err(format!("Invalid platform '{platform:#?}'"))
}

#[tauri::command]
pub async fn refresh_feed(
    app_handle: AppHandle,
    state: State<'_, Mutex<AppState>>,
    platform: Platform,
) -> Result<(), String> {
    let state = state.lock().await;
    let users_db = state.users_db.as_ref().unwrap();
    let feeds_db = state.feeds_db.as_ref().unwrap();

    if platform == Platform::Twitch {
        let query = "SELECT username FROM twitch";

        let rows = match sqlx::query(query).fetch_all(users_db).await {
            Ok(rows) => rows,
            Err(err) => {
                return Err(format!("Querying usernames: {err}"));
            }
        };

        let mut usernames: Vec<String> = Vec::with_capacity(rows.len());

        for row in rows {
            let username = row.get::<String, _>(0);
            usernames.push(username);
        }

        let live_now = match twitch::stream::fetch_live_now(usernames).await {
            Ok(live_now) => live_now,
            Err(err) => {
                return Err(format!("Fetching live now: {err}"));
            }
        };

        let query = "DELETE FROM twitch";
        if let Err(err) = sqlx::query(query).execute(feeds_db).await {
            return Err(format!("Executing delete query: {err}"));
        }

        let query = "INSERT INTO twitch (username, started_at) VALUES (?, ?)";

        for (username, live) in live_now {
            if let Err(err) = sqlx::query(query)
                .bind(&username)
                .bind(&live.started_at)
                .execute(feeds_db)
                .await
            {
                error!("Executing insert query: {err}");
            }
        }

        if let Err(err) = app_handle.emit("updated_streams", &platform) {
            return Err(format!("Emitting 'updated_streams' event: {err}"));
        }
    }

    if platform == Platform::YouTube {
        let query = "SELECT id FROM youtube";

        let rows = match sqlx::query(query).fetch_all(users_db).await {
            Ok(rows) => rows,
            Err(err) => {
                return Err(format!("Querying channel ids: {err}"));
            }
        };

        let mut channel_ids: Vec<String> = Vec::new();

        for row in rows {
            let channel_id = row.get::<String, _>(0);
            channel_ids.push(channel_id);
        }

        let videos = match youtube::video::fetch_videos(channel_ids).await {
            Ok(videos) => videos,
            Err(err) => {
                return Err(format!("Requesting videos: {err}"));
            }
        };

        let query = "DELETE FROM youtube";
        if let Err(err) = sqlx::query(query).execute(feeds_db).await {
            return Err(format!("Executing delete query: {err}"));
        }

        let mut tx = feeds_db
            .begin()
            .await
            .map_err(|err| format!("Beginning transaction: {err}"))?;

        let placeholders = vec!["(?, ?, ?, ?, ?)"; videos.len()].join(", ");

        let sql = format!("INSERT INTO youtube (id, username, title, published_at, view_count) VALUES {placeholders}");

        let mut query = sqlx::query(&sql);

        for video in &videos {
            query = query
                .bind(&video.id)
                .bind(&video.username)
                .bind(&video.title)
                .bind(video.published_at)
                .bind(&video.view_count);
        }

        if let Err(err) = query.execute(&mut *tx).await {
            return Err(format!("Executing insert query: {err}"));
        }

        if let Err(err) = tx.commit().await {
            return Err(format!("Committing transaction: {err}"));
        }

        if let Err(err) = app_handle.emit("updated_videos", platform) {
            return Err(format!("Emitting 'updated_videos' event: {err}"));
        }
    }

    Ok(())
}
