CREATE TABLE oauth_tokens (
    id INTEGER PRIMARY KEY,
    credentials_id INTEGER NOT NULL,
    access_token TEXT NOT NULL,
    refresh_token TEXT NOT NULL,
    expires_at TIMESTAMP NOT NULL,
    scope TEXT,
    FOREIGN KEY (credentials_id) REFERENCES service_credentials (id) ON DELETE CASCADE
);