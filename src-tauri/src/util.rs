use std::time::{SystemTime, UNIX_EPOCH};

use log::error;

use crate::twitch::main::HTTP_CLIENT;

pub fn random_number(start: u32, end: u32) -> u32 {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .subsec_nanos();

    start + (nanos % (end - start))
}

pub async fn download_image(url: &str) -> Vec<u8> {
    if url.is_empty() {
        error!("Image URL is empty");
        return Vec::new();
    }

    let response = match HTTP_CLIENT.get(url).send().await {
        Ok(response) => response,
        Err(err) => {
            error!("Requesting image: {err}");
            return Vec::new();
        }
    };

    if !response.status().is_success() {
        error!("Downloading image: {url}");
        return Vec::new();
    }

    match response.bytes().await {
        Ok(bytes) => bytes.to_vec(),
        Err(err) => {
            error!("Reading image bytes: {err}");
            Vec::new()
        }
    }
}
