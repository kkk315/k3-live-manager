-- Add unique constraint to ensure only one token per credentials_id
CREATE UNIQUE INDEX idx_oauth_tokens_credentials_id ON oauth_tokens(credentials_id);
