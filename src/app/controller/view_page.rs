use axum::response::Html;
use serde_json::json;

use crate::{app::App, error::Error, query::page::Page};

pub async fn run(title: String, app: App) -> Result<Html<String>, Error> {
    if let Some(response) = view_page(&title, &app).await? {
        Ok(response)
    } else {
        page_not_found(&title, &app).await
    }
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
