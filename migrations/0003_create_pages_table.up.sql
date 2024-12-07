CREATE TABLE IF NOT EXISTS pages (
    page_id SERIAL PRIMARY KEY,
    page_title VARCHAR(255) UNIQUE NOT NULL,

    page_rev INTEGER NOT NULL
        REFERENCES revisions
);