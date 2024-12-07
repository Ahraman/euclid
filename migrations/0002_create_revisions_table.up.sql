CREATE TABLE IF NOT EXISTS revisions (
    rev_id SERIAL PRIMARY KEY,
    rev_parent INTEGER DEFAULT NULL
        REFERENCES revisions,

    rev_content INTEGER NOT NULL
        REFERENCES contents,
    rev_page INTEGER NOT NULL,
    rev_time TIMESTAMPTZ NOT NULL
        DEFAULT NOW()
);