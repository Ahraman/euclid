use sqlx::{query, query_as, PgExecutor};

use crate::error::Error;

pub struct Page {
    pub id: i32,
    pub rev_id: i32,
}

impl Page {
    pub async fn create(
        title: &str,
        content: &Content,
        conn: impl PgExecutor<'_>,
    ) -> Result<Self, Error> {
        Ok(query_as!(
            Page,
            r#"
                WITH page (page_id, rev_id) AS (
                        INSERT INTO pages (page_title, page_rev)
                            VALUES ($1, nextval('revisions_rev_id_seq'))
                            RETURNING page_id, page_rev
                    )
                    INSERT INTO revisions (rev_id, rev_content, rev_page)
                        SELECT rev_id, $2, page_id
                            FROM page
                        RETURNING rev_page AS id, rev_page AS rev_id
                    "#,
            &title,
            content.id
        )
        .fetch_one(conn)
        .await?)
    }

    pub async fn find(title: &str, conn: impl PgExecutor<'_>) -> Result<Option<Self>, Error> {
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

    pub async fn update_revision(
        &self,
        revision: &Revision,
        conn: impl PgExecutor<'_>,
    ) -> Result<(), Error> {
        query!(
            r#"
            UPDATE pages
                SET page_rev = $2
                WHERE page_id = $1
            "#,
            self.id,
            revision.id,
        )
        .execute(conn)
        .await?;

        Ok(())
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
    pub async fn create(
        page: &Page,
        content: &Content,
        conn: impl PgExecutor<'_>,
    ) -> Result<Self, Error> {
        Ok(query_as!(
            Revision,
            r#"
                    INSERT INTO revisions (rev_parent, rev_content, rev_page)
                        VALUES ($2, $3, $1)
                        RETURNING rev_id AS id, rev_content AS content_id
                    "#,
            page.id,
            page.rev_id,
            content.id,
        )
        .fetch_one(conn)
        .await?)
    }

    pub async fn fetch(page: &Page, conn: impl PgExecutor<'_>) -> Result<Self, Error> {
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
    pub async fn fetch(revision: &Revision, conn: impl PgExecutor<'_>) -> Result<Self, Error> {
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

    pub async fn insert(content: String, conn: impl PgExecutor<'_>) -> Result<Content, Error> {
        Ok(query_as!(
            Content,
            r#"INSERT INTO contents (content_text)
                    VALUES ($1)
                    RETURNING content_id AS id, content_text AS text"#,
            &content
        )
        .fetch_one(conn)
        .await?)
    }
}
