-- Add migration script here
CREATE TABLE IF NOT EXISTS instagram_comments (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  instagram_user_id BIGINT NOT NULL, 
  comment_date TIMESTAMP WITH TIME ZONE,
  comment_from_subscriber_id TEXT,
  comment_from_subscriber_username TEXT,
  comment_media_id TEXT,
  comment_media_product_type TEXT,
  comment_id TEXT NOT NULL,
  comment_text TEXT NOT NULL,
  CONSTRAINT fk_instagram
    FOREIGN KEY (instagram_user_id)
    REFERENCES instagram(instagram_user_id)
    ON DELETE CASCADE
)
