use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
};
use serde::Deserialize;

use crate::{
    app::{
        controller::{edit_page, view_page},
        App,
    },
    error::Error,
};

#[derive(Default, Clone, Copy, Deserialize)]
pub enum Action {
    #[serde(rename = "view")]
    #[default]
    View,
    #[serde(rename = "edit")]
    Edit,
}

#[derive(Deserialize)]
pub struct PageInfo {
    #[serde(default)]
    pub action: Action,
}

pub async fn get(
    Path(title): Path<String>,
    State(app): State<App>,
    Query(query): Query<PageInfo>,
) -> Result<impl IntoResponse, Error> {
    match query.action {
        Action::View => view_page::run(title, app).await,
        Action::Edit => edit_page::run(title, app).await,
    }
}
