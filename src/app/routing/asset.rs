use axum::{extract::Query, response::IntoResponse};
use serde::Deserialize;

use crate::{app::controller::load_asset, error::Error};

#[derive(Default, Clone, Copy, Deserialize)]
pub enum AssetKind {
    #[default]
    Unknown,
    #[serde(rename = "css")]
    Css,
}

impl AssetKind {
    pub fn content_type(&self) -> &'static str {
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

pub async fn get(Query(query): Query<AssetInfo>) -> Result<impl IntoResponse, Error> {
    load_asset::run(query.path, query.kind).await
}
