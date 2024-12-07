use axum::{routing::get, Router};
use handlebars::Handlebars;
use routing::{asset, page, root};
use sqlx::PgPool;
use tokio::fs;

pub mod controller;
pub mod routing;

use crate::error::Error;

#[derive(Clone)]
pub struct App {
    pub handlebars: Handlebars<'static>,

    pub conn: PgPool,
}

impl App {
    pub async fn new() -> Result<Self, Error> {
        Ok(Self {
            handlebars: Self::create_handlebars().await?,

            conn: PgPool::connect(&std::env::var("DATABASE_URL")?).await?,
        })
    }

    async fn create_handlebars() -> Result<Handlebars<'static>, Error> {
        let mut handlebars = Handlebars::new();
        handlebars.register_partial(
            "base-page",
            fs::read_to_string("assets/templates/base-page.handlebars").await?,
        )?;
        handlebars.register_template_string(
            "view-page",
            fs::read_to_string("assets/templates/view-page.handlebars").await?,
        )?;
        handlebars.register_template_string(
            "edit-page",
            fs::read_to_string("assets/templates/edit-page.handlebars").await?,
        )?;
        handlebars.register_template_string(
            "page-not-found",
            fs::read_to_string("assets/templates/page-not-found.handlebars").await?,
        )?;
        Ok(handlebars)
    }

    pub fn into_router(self) -> Router {
        Router::new()
            .route("/", get(root::get))
            .route("/w/", get(page::get))
            .route("/w/:title", get(page::get).post(page::post))
            .route("/asset", get(asset::get))
            .with_state(self)
    }
}
