-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  username TEXT,
  birthday TIMESTAMP WITH TIME ZONE,
  first_name TEXT,
  last_name TEXT,
  gender TEXT,
  image_url TEXT,
  email_address TEXT,
  clerk_user_id TEXT UNIQUE,
  created_at TIMESTAMP WITH TIME ZONE,
  updated_at TIMESTAMP WITH TIME ZONE
);

