ALTER TABLE IF EXISTS revisions
    ADD CONSTRAINT revisions_rev_page_fkey FOREIGN KEY (rev_page)
        REFERENCES pages;