-- Add migration script here
CREATE TABLE Agent(
    id TEXT NOT NULL UNIQUE,
    implant INTERGER NOT NULL,
    created_at TEXT NOT NULL, 
    last_seen TEXT ,
    os TEXT NOT NULL,
    ip TEXT NOT NULL,
    username TEXT NOT NULL,
    hostname TEXT NOT NULL
);
