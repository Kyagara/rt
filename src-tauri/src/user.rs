use anyhow::Result;

use log::error;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Row, Sqlite};
use tauri::{async_runtime::Mutex, AppHandle, Emitter, State};

use crate::{twitch, youtube, AppState};

#[derive(Serialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub platform: Platform,
    pub avatar: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
pub enum Platform {
    #[serde(rename = "youtube")]
    YouTube,
    #[serde(rename = "twitch")]
    Twitch,
}

#[tauri::command]
pub async fn get_users(
    state: State<'_, Mutex<AppState>>,
    platform: Option<Platform>,
) -> Result<Vec<User>, String> {
    let state = state.lock().await;
    let users_db = state.users_db.as_ref().unwrap();

    if let Some(platform) = platform {
        if let Ok(users) = get_users_for_platform(users_db, platform).await {
            Ok(users)
        } else {
            Err(format!("Getting user for {platform:#?}"))
        }
    } else {
        let mut users = Vec::new();

        match get_users_for_platform(users_db, Platform::YouTube).await {
            Ok(new_users) => users.extend(new_users),
            Err(err) => {
                return Err(format!("Getting YouTube users: {err}"));
            }
        }

        match get_users_for_platform(users_db, Platform::Twitch).await {
            Ok(new_users) => users.extend(new_users),
            Err(err) => {
                return Err(format!("Getting Twitch users: {err}"));
            }
        }

        Ok(users)
    }
}

#[tauri::command]
pub async fn add_user(
    app_handle: AppHandle,
    state: State<'_, Mutex<AppState>>,
    username: String,
    platform: Platform,
) -> Result<(), String> {
    let state = state.lock().await;
    let users_db = state.users_db.as_ref().unwrap();
    let emotes_db = state.emotes_db.as_ref().unwrap();

    if platform == Platform::Twitch {
        let (user, emotes) = match twitch::user::fetch_user(&username).await {
            Ok(user) => user,
            Err(err) => {
                return Err(format!("Fetching user '{username}': {err}"));
            }
        };

        if let Err(err) = twitch::emote::update_user_emotes(emotes_db, &username, emotes).await {
            error!("Saving emotes for user '{username}': {err}");
        }

        let query = "INSERT INTO twitch (id, username, avatar) VALUES (?, ?, ?) ON CONFLICT (username) DO UPDATE SET avatar = ?";

        if let Err(err) = sqlx::query(query)
            .bind(&user.id)
            .bind(&user.username)
            .bind(&user.avatar)
            .bind(&user.avatar)
            .execute(users_db)
            .await
        {
            return Err(format!("Saving user: {err}"));
        }
    }

    if platform == Platform::YouTube {
        let user = match youtube::channel::fetch_channel_by_name(&username).await {
            Ok(user) => user,
            Err(err) => {
                return Err(format!("Fetching user '{username}': {err}"));
            }
        };

        let query = "INSERT INTO youtube (id, username, avatar) VALUES (?, ?, ?) ON CONFLICT (username) DO UPDATE SET avatar = ?";

        if let Err(err) = sqlx::query(query)
            .bind(&user.id)
            .bind(&user.username)
            .bind(&user.avatar)
            .bind(&user.avatar)
            .execute(users_db)
            .await
        {
            return Err(format!("Saving user: {err}"));
        }
    }

    if let Err(err) = app_handle.emit("updated_users", platform) {
        error!("Emitting 'updated_users' event: {err}");
    }

    Ok(())
}

#[tauri::command]
pub async fn remove_user(
    app_handle: AppHandle,
    state: State<'_, Mutex<AppState>>,
    username: String,
    platform: Platform,
) -> Result<(), String> {
    let state = state.lock().await;
    let users_db = state.users_db.as_ref().unwrap();
    let feeds_db = state.feeds_db.as_ref().unwrap();
    let emotes_db = state.emotes_db.as_ref().unwrap();

    if platform == Platform::Twitch {
        let query = "DELETE FROM twitch WHERE username = ?";
        if let Err(err) = sqlx::query(query).bind(&username).execute(users_db).await {
            return Err(format!("Deleting user: {err}"));
        }

        let query = "DELETE FROM twitch WHERE username = ?";
        if let Err(err) = sqlx::query(query).bind(&username).execute(feeds_db).await {
            return Err(format!("Deleting feed: {err}"));
        }

        let query = "DELETE FROM twitch WHERE username = ?";
        if let Err(err) = sqlx::query(query).bind(&username).execute(emotes_db).await {
            return Err(format!("Deleting emotes: {err}"));
        }
    }

    if platform == Platform::YouTube {
        let query = "DELETE FROM youtube WHERE username = ?";
        if let Err(err) = sqlx::query(query).bind(&username).execute(users_db).await {
            return Err(format!("Deleting user: {err}"));
        }

        let query = "DELETE FROM youtube WHERE username = ?";
        if let Err(err) = sqlx::query(query).bind(&username).execute(feeds_db).await {
            return Err(format!("Deleting feed: {err}"));
        }
    }

    if let Err(err) = app_handle.emit("updated_users", platform) {
        return Err(format!("Error emitting 'updated_users' event: {err}"));
    }

    Ok(())
}

async fn get_users_for_platform(users_db: &Pool<Sqlite>, platform: Platform) -> Result<Vec<User>> {
    let query = match platform {
        Platform::YouTube => "SELECT id, username, avatar FROM youtube",
        Platform::Twitch => "SELECT id, username, avatar FROM twitch",
    };

    let rows = sqlx::query(query)
        .fetch_all(users_db)
        .await
        .expect("Querying users");

    let mut users: Vec<User> = Vec::with_capacity(rows.len());

    for row in rows {
        let user = User {
            id: row.get::<String, _>(0),
            username: row.get::<String, _>(1),
            avatar: row.get::<Vec<u8>, _>(2),
            platform,
        };

        users.push(user);
    }

    Ok(users)
}
