-- Add migration script here
CREATE TABLE IF NOT EXISTS instagram (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  username TEXT UNIQUE NOT NULL,
  instagram_user_id BIGINT UNIQUE NOT NULL,
  instagram_access_token TEXT,
  instagram_access_token_expires_at TIMESTAMP WITH TIME ZONE,
  clerk_user_id TEXT UNIQUE,
  profile_picture_url TEXT,
  CONSTRAINT fk_users
    FOREIGN KEY(clerk_user_id)
    REFERENCES users(clerk_user_id)
    ON DELETE CASCADE
);
