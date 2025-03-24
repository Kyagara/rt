use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use log::{error, info};
use regex::Regex;
use tauri::{
    async_runtime::{self, Mutex},
    AppHandle, Theme, WebviewUrl, WebviewWindowBuilder,
};

lazy_static! {
    static ref WINDOW_ID: Mutex<u64> = Mutex::new(0);
    static ref TWITCH_URL_REG: Regex = Regex::new(r"twitch.tv/([a-zA-Z0-9_]+)").unwrap();
    // https://stackoverflow.com/a/37704433
    static ref YOUTUBE_URL_REG: Regex = Regex::new(r"^((?:https?:)?\/\/)?((?:www|m)\.)?((?:youtube(?:-nocookie)?\.com|youtu.be))(\/(?:[\w\-]+\?v=|embed\/|v\/)?)([\w\-]+)(\S+)?$").unwrap();
}

pub fn open_url(app_handle: AppHandle, urls: &[String]) -> Result<()> {
    if urls.is_empty() {
        return Err(anyhow!("No URLs provided"));
    }

    let url = urls.first().unwrap();

    // For Twitch, only streams are supported, so just get the username from the URL
    if url.starts_with("rt://tw/") || url.starts_with("rt://twitch/") {
        let username = url
            .trim_start_matches("rt://tw/")
            .trim_start_matches("rt://twitch/");
        let url = format!("/streams/watch?username={username}");
        open_new_window(app_handle, url);
        return Ok(());
    }

    if let Some(caps) = TWITCH_URL_REG.captures(url) {
        if let Some(m) = caps.get(1) {
            let username = m.as_str();
            let url = format!("/streams/watch?username={username}");
            open_new_window(app_handle, url);
            return Ok(());
        }
    }

    // For YouTube, only URLs with video IDs are supported
    if url.starts_with("rt://yt/") || url.starts_with("rt://youtube/") {
        let video_id = url
            .trim_start_matches("rt://yt/")
            .trim_start_matches("rt://youtube/");
        let url = format!("/videos/watch?id={video_id}");
        open_new_window(app_handle, url);
        return Ok(());
    }

    if let Some(caps) = YOUTUBE_URL_REG.captures(url) {
        if let Some(m) = caps.get(5) {
            let video_id = m.as_str();
            let url = format!("/videos/watch?id={video_id}");
            open_new_window(app_handle, url);
            return Ok(());
        }
    }

    Ok(())
}

#[tauri::command]
pub fn open_new_window(app_handle: AppHandle, url: String) {
    info!("Opening new window for '{url}'");

    // In Windows, a new window must be created in a separate thread
    async_runtime::spawn(async move {
        let webview_url = WebviewUrl::App(url.into());

        let mut window_id = WINDOW_ID.lock().await;
        *window_id += 1;

        if let Err(err) =
            WebviewWindowBuilder::new(&app_handle, format!("rt-{window_id}"), webview_url)
                .theme(Some(Theme::Dark))
                .shadow(false)
                .decorations(false)
                .build()
        {
            error!("Error creating new window: {err}");
        }
    });
}
