use anyhow::anyhow;
use log::{error, LevelFilter};
use sqlx::SqlitePool;
use tauri::{
    async_runtime::{self, Mutex},
    Manager,
};
use tauri_plugin_deep_link::DeepLinkExt;

mod feed;
mod migration;
mod twitch;
mod user;
mod util;
mod window;
mod youtube;

pub struct AppState {
    pub users_db: Option<SqlitePool>,
    pub feeds_db: Option<SqlitePool>,
    pub emotes_db: Option<SqlitePool>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default();

    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(
            |app_handle, args, _cwd| {
                let app_handle = app_handle.clone();

                // Remove first element from the args, it's the executable path
                let urls = &args[1..];

                if let Err(err) = window::open_url(app_handle, urls) {
                    error!("Opening new window: {err}");
                };
            },
        ));
    }

    builder = builder
        .plugin(
            tauri_plugin_sql::Builder::new()
                .add_migrations("sqlite:users.db", migration::users_migrations())
                .add_migrations("sqlite:feeds.db", migration::feeds_migrations())
                .add_migrations("sqlite:emotes.db", migration::emotes_migrations())
                .build(),
        )
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(LevelFilter::Debug)
                .level_for("rustls", LevelFilter::Off)
                .level_for("h2", LevelFilter::Off)
                .level_for("hyper_util", LevelFilter::Off)
                .level_for("sqlx", LevelFilter::Info)
                .level_for(
                    "tao::platform_impl::platform::event_loop::runner",
                    LevelFilter::Off,
                )
                .build(),
        )
        .setup(|app| {
            #[cfg(desktop)]
            app.deep_link().register("rt")?;

            let mut app_data_dir = app.path().app_data_dir()?;

            let platform = std::env::consts::OS;
            if platform == "linux" {
                app_data_dir = app.path().app_config_dir()?;
            }

            async_runtime::block_on(async {
                let storage_dir = app_data_dir.join("rustypipe");
                if let Err(err) = youtube::main::build_client(&storage_dir).await {
                    return Err(anyhow!("Building new RustyPipe client: {err}"));
                }

                let users_db_path = app_data_dir.join("users.db");
                let users_db = match SqlitePool::connect(users_db_path.to_str().unwrap()).await {
                    Ok(db) => db,
                    Err(err) => {
                        return Err(anyhow!("Connecting to 'users.db': {err}"));
                    }
                };

                let feeds_db_path = app_data_dir.join("feeds.db");
                let feeds_db = match SqlitePool::connect(feeds_db_path.to_str().unwrap()).await {
                    Ok(db) => db,
                    Err(err) => {
                        return Err(anyhow!("Connecting to 'feeds.db': {err}"));
                    }
                };

                let emotes_db_path = app_data_dir.join("emotes.db");
                let emotes_db = match SqlitePool::connect(emotes_db_path.to_str().unwrap()).await {
                    Ok(db) => db,
                    Err(err) => {
                        return Err(anyhow!("Connecting to 'emotes.db': {err}"));
                    }
                };

                Ok(app.manage(Mutex::new(AppState {
                    users_db: Some(users_db),
                    feeds_db: Some(feeds_db),
                    emotes_db: Some(emotes_db),
                })))
            })?;

            Ok(())
        });

    builder
        .invoke_handler(tauri::generate_handler![
            user::get_users,
            user::add_user,
            user::remove_user,
            feed::get_feed,
            feed::refresh_feed,
            window::open_new_window,
            twitch::stream::fetch_stream_playback,
            twitch::proxy::proxy_stream,
            twitch::chat::join_chat,
            youtube::main::import_subscriptions,
        ])
        .run(tauri::generate_context!())
        .expect("while running tauri application");
}
