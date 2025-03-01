-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  username TEXT,
  birthday TIMESTAMP,
  first_name TEXT,
  last_name TEXT,
  gender TEXT,
  image_url TEXT,
  email_address TEXT,
  clerk_user_id TEXT,
  created_at TIMESTAMP
);


DO $$
BEGIN
if not exists (select constraint_name 
                   from information_schema.constraint_column_usage 
                   where table_name = 'users'  and constraint_name = 'users_unique_email_address_clerk_user_id' and column_name IN ('email_address', 'clerk_user_id')) then 
                    ALTER TABLE users
ADD CONSTRAINT users_unique_email_address_clerk_user_id UNIQUE (email_address, clerk_user_id);
end if;
                    END $$;
