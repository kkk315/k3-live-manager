CREATE TABLE service_credentials (
    id INTEGER PRIMARY KEY,
    service_name TEXT NOT NULL UNIQUE,
    client_id TEXT NOT NULL,
    client_secret TEXT NOT NULL
);