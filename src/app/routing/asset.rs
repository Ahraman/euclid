use std::path::Path;

use axum::{extract::Query, response::Response};
use serde::Deserialize;
use tokio::fs;

use crate::error::Error;

#[derive(Default, Clone, Copy, Deserialize)]
pub enum AssetKind {
    #[default]
    Unknown,
    #[serde(rename = "css")]
    Css,
}

impl AssetKind {
    fn content_type(&self) -> &'static str {
        match self {
            AssetKind::Unknown => "application/octet-stream",
            AssetKind::Css => "text/css",
        }
    }
}

#[derive(Deserialize)]
pub struct AssetInfo {
    pub path: String,
    #[serde(rename = "type")]
    pub kind: AssetKind,
}

pub async fn get(Query(query): Query<AssetInfo>) -> Result<Response, Error> {
    let kind = if let AssetKind::Unknown = query.kind {
        if let Some(extension) = Path::new(&query.path)
            .extension()
            .map(|s| s.to_str())
            .flatten()
        {
            match extension {
                "css" => AssetKind::Css,
                _ => AssetKind::Unknown,
            }
        } else {
            AssetKind::Unknown
        }
    } else {
        query.kind
    };

    Ok(Response::builder()
        .header("Content-Type", kind.content_type())
        .body(
            fs::read_to_string(format!("assets/{}", &query.path))
                .await?
                .into(),
        )?)
}
