use axum::response::{Html, IntoResponse, Response};
use serde_json::json;

use crate::{app::App, error::Error, query::page::Page};

pub async fn run(title: String, app: App) -> Result<Response, Error> {
    Ok(if let Some(html) = view_page(&title, &app).await? {
        html.into_response()
    } else {
        page_not_found(&title, &app).await?.into_response()
    })
}

async fn view_page(title: &str, app: &App) -> Result<Option<Html<String>>, Error> {
    Ok(if let Some(page) = Page::find(title, &app.conn).await? {
        let content = page
            .revision(&app.conn)
            .await?
            .content(&app.conn)
            .await?
            .text;

        Some(
            app.handlebars
                .render(
                    "view-page",
                    &json!({
                        "page": {
                            "title": title,
                            "content": content,
                        }
                    }),
                )?
                .into(),
        )
    } else {
        None
    })
}

async fn page_not_found(title: &str, app: &App) -> Result<Html<String>, Error> {
    Ok(app
        .handlebars
        .render(
            "page-not-found",
            &json!(
                {
                    "page": {
                        "title": title
                    }
                }
            ),
        )?
        .into())
}
