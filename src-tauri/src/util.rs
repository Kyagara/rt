use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Result;
use log::error;

use crate::twitch::main::HTTP_CLIENT;

pub fn random_number(start: u32, end: u32) -> u32 {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .subsec_nanos();

    start + (nanos % (end - start))
}

pub async fn download_image(url: &str) -> Result<Vec<u8>> {
    if url.is_empty() {
        error!("Failed to download image: url is empty");
        return Ok(Vec::new());
    }

    let response = HTTP_CLIENT.get(url).send().await?;

    if !response.status().is_success() {
        error!("Failed to download image: {url}");
        return Ok(Vec::new());
    }

    let bytes = response.bytes().await.expect("Failed to get image bytes");

    Ok(bytes.to_vec())
}
