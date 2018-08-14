CREATE TABLE entry (
    id TEXT PRIMARY KEY,
    author TEXT NOT NULL,
    published INTEGER NOT NULL,
    data TEXT NOT NULL
) WITHOUT ROWID ;

CREATE INDEX fk_entry_author_published ON entry(author, published) ;
