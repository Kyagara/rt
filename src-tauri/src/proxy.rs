use std::borrow::Cow;

use axum::{
    body::{Body, HttpBody},
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use bytes::Bytes;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use tauri::Url;
use tauri_plugin_http::reqwest::{header::HeaderValue, Client};

use crate::api::LOCAL_API;

lazy_static! {
    static ref HTTP_CLIENT: Client = Client::new();
}

#[derive(Serialize, Deserialize, Default)]
pub struct ProxyStreamQuery {
    url: String,
}

enum M3U8Result {
    Success(String),
}

pub async fn proxy(url: Query<ProxyStreamQuery>) -> impl IntoResponse {
    let Query(query) = url;

    if query.url.is_empty() {
        return (StatusCode::BAD_REQUEST, Response::default());
    }

    let resp = match HTTP_CLIENT.get(query.url.as_str()).send().await {
        Ok(resp) => resp,
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Response::new(format!("Error proxying request: {}", err).into()),
            );
        }
    };

    let mut headers = resp.headers().clone();
    let content_type = headers
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default();

    let body = match content_type {
        "application/vnd.apple.mpegurl" => {
            let body_bytes = resp.bytes().await.unwrap_or(Bytes::new());

            let M3U8Result::Success(body) =
                process_m3u8(query.url, String::from_utf8_lossy(&body_bytes));

            Body::from(body)
        }
        _ => Body::from(resp.bytes().await.unwrap_or(Bytes::new())),
    };

    let mut response = Response::new(Body::default());

    if content_type.contains("stream") {
        *response.body_mut() = Body::from_stream(body.into_data_stream())
    } else {
        let new_content_length = match body.size_hint().exact() {
            Some(size) => size.to_string(),
            None => "0".to_string(),
        };

        headers.insert(
            "content-length",
            HeaderValue::from_str(&new_content_length).unwrap(),
        );

        *response.body_mut() = body;
    }

    *response.headers_mut() = headers.clone();

    (StatusCode::OK, response)
}

fn process_m3u8(base_url: String, playlist: Cow<'_, str>) -> M3U8Result {
    let base_url = Url::parse(&base_url).ok();
    let reg = Regex::new(r"^(https?://[^\s]+)").unwrap();

    let result = playlist
        .lines()
        .map(|line| {
            // Add PROXY_URL to all urls
            if reg.is_match(line) || (!line.starts_with("#") && !line.is_empty()) {
                if let Some(base) = &base_url {
                    if let Ok(abs_url) = base.join(line) {
                        return format!(
                            "{LOCAL_API}/proxy?url={}",
                            urlencoding::encode(abs_url.as_str())
                        );
                    }
                }
            }

            line.to_string()
        })
        .collect::<Vec<_>>()
        .join("\n");

    M3U8Result::Success(result)
}
