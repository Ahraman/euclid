use axum::response::{Html, IntoResponse, Response};
use serde_json::json;

use crate::{app::App, error::Error, query::page::Page};

pub async fn run(title: String, app: App) -> Result<Response, Error> {
    Ok(edit_page(&title, &app).await?.into_response())
}

async fn edit_page(title: &str, app: &App) -> Result<Html<String>, Error> {
    let content = if let Some(page) = Page::find(title, &app.conn).await? {
        page.revision(&app.conn)
            .await?
            .content(&app.conn)
            .await?
            .text
    } else {
        Default::default()
    };

    Ok(app
        .handlebars
        .render(
            "edit-page",
            &json!({
                "page": {
                    "title": title,
                    "content": content,
                }
            }),
        )?
        .into())
}
