use axum::response::{IntoResponse, Redirect, Response};

use crate::{
    app::App,
    error::Error,
    query::page::{Content, Page, Revision},
};

pub async fn run(title: String, content: String, app: App) -> Result<Response, Error> {
    Ok(submit_page(&title, content, &app).await?.into_response())
}

async fn submit_page(title: &str, content: String, app: &App) -> Result<impl IntoResponse, Error> {
    let content = Content::insert(content, &app.conn).await?;

    if let Some(page) = Page::find(&title, &app.conn).await? {
        let mut tx = app.conn.begin().await?;
        let revision = Revision::create(&page, &content, &mut *tx).await?;
        page.update_revision(&revision, &mut *tx).await?;
        tx.commit().await?;
    } else {
        let _ = Page::create(title, &content, &app.conn).await?;
    };

    Ok(Redirect::to(&format!("/w/{title}")))
}
