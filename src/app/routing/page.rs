use axum::{
    extract::{Path, Query, State},
    response::Response,
    Form,
};
use serde::Deserialize;

use crate::{
    app::{
        controller::{edit_page, submit_page, view_page},
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
    #[serde(rename = "submit")]
    Submit,
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
) -> Result<Response, Error> {
    match query.action {
        Action::View | Action::Submit => view_page::run(title, app).await,
        Action::Edit => edit_page::run(title, app).await,
    }
}

#[derive(Deserialize)]
pub struct SubmitPageInfo {
    content: String,
}

pub async fn post(
    Path(title): Path<String>,
    State(app): State<App>,
    Query(query): Query<PageInfo>,
    Form(form): Form<SubmitPageInfo>,
) -> Result<Response, Error> {
    match query.action {
        Action::View => view_page::run(title, app).await,
        Action::Edit => edit_page::run(title, app).await,
        Action::Submit => submit_page::run(title, form.content, app).await,
    }
}
