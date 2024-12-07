use std::path::Path;

use axum::response::Response;
use tokio::fs;

use crate::{app::routing::asset::AssetKind, error::Error};

pub async fn run(path: String, kind: AssetKind) -> Result<Response, Error> {
    load_asset(&path, kind).await
}

async fn load_asset(path: &str, kind: AssetKind) -> Result<Response, Error> {
    let kind = if let AssetKind::Unknown = kind {
        if let Some(extension) = Path::new(&path).extension().map(|s| s.to_str()).flatten() {
            match extension {
                "css" => AssetKind::Css,
                _ => AssetKind::Unknown,
            }
        } else {
            AssetKind::Unknown
        }
    } else {
        kind
    };

    Ok(Response::builder()
        .header("Content-Type", kind.content_type())
        .body(fs::read_to_string(format!("assets/{path}")).await?.into())?)
}
