use std::{net::IpAddr, str::FromStr};

use axum::http::HeaderMap;
use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine};
use regex::Regex;
use sha2::{Digest, Sha256};

/// Extracts the client's IP Address from CloudFlare's CF-Connecting-IP
/// HTTP header, and generates a BASE64 SHA256 hash with it.
///
/// Of course this makes the assumption that the service is hosted
/// behind CloudFlare. :)
pub fn extract_source_hash(headers: &HeaderMap) -> Option<String> {
    headers
        .get("CF-Connecting-IP")
        .filter(|ip| IpAddr::from_str(ip.to_str().unwrap_or_default()).is_ok())
        .map(|ip| {
            let mut hasher = Sha256::new();
            hasher.update(ip.as_bytes());
            let hash = hasher.finalize();
            BASE64_STANDARD.encode(&hash)
        })
}

pub fn is_steamid3(data: &str) -> bool {
    Regex::new(r"\[U:1:\d+]").unwrap().is_match(data)
}
