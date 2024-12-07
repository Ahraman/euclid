use sqlx::{query_as, PgExecutor};

use crate::error::Error;

pub struct Page {
    pub id: i32,
    pub rev_id: i32,
}

impl Page {
    pub async fn find(title: &str, conn: impl PgExecutor<'_>) -> Result<Option<Page>, Error> {
        Ok(query_as!(
            Page,
            r#"SELECT page_id AS id, page_rev AS rev_id
                FROM pages
                WHERE page_title = $1"#,
            &title
        )
        .fetch_optional(conn)
        .await?)
    }

    pub async fn revision(&self, conn: impl PgExecutor<'_>) -> Result<Revision, Error> {
        Revision::fetch(self, conn).await
    }
}

pub struct Revision {
    pub id: i32,
    pub content_id: i32,
}

impl Revision {
    pub async fn fetch(page: &Page, conn: impl PgExecutor<'_>) -> Result<Revision, Error> {
        Ok(query_as!(
            Revision,
            r#"SELECT rev_id AS id, rev_content AS content_id
                FROM revisions
                WHERE rev_id = $1"#,
            page.rev_id
        )
        .fetch_one(conn)
        .await?)
    }

    pub async fn content(&self, conn: impl PgExecutor<'_>) -> Result<Content, Error> {
        Content::fetch(self, conn).await
    }
}

pub struct Content {
    pub id: i32,
    pub text: String,
}

impl Content {
    pub async fn fetch(revision: &Revision, conn: impl PgExecutor<'_>) -> Result<Content, Error> {
        Ok(query_as!(
            Content,
            r#"SELECT content_id AS id, content_text AS text
                FROM contents
                WHERE content_id = $1"#,
            revision.content_id
        )
        .fetch_one(conn)
        .await?)
    }
}
