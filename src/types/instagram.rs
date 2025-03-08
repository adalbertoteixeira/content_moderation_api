use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstagramMe {
    pub id: Option<String>,
    pub user_id: Option<String>,
    pub username: Option<String>,
    pub profile_picture_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstagramAccessToken {
    pub instagram_user_id: Option<i64>,
    pub clerk_user_id: Option<String>,
    pub access_token: Option<String>,
    pub token_type: Option<String>,
    pub expires_in: Option<i32>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Instagram {
    pub id: Option<Uuid>,
    pub username: Option<String>,
    pub instagram_user_id: Option<i64>,
    pub instagram_access_token: Option<String>,
    pub instagram_access_token_expires_at: Option<DateTime<Utc>>,
    pub clerk_user_id: Option<String>,
    pub profile_picture_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstagramWebhookEntryValueFrom {
    pub id: Option<String>,
    pub username: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstagramWebhookEntryValueMedia {
    pub id: Option<String>,
    pub media_product_type: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstagramWebhookEntryValue {
    pub from: Option<InstagramWebhookEntryValueFrom>,
    pub media: Option<InstagramWebhookEntryValueMedia>,
    pub id: Option<String>,
    pub parent_id: Option<String>,
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstagramWebhookEntryChange {
    pub field: Option<String>,
    pub value: Option<InstagramWebhookEntryValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstagramWebhookEntry {
    pub id: Option<String>,
    pub time: Option<i64>,
    pub changes: Option<Vec<InstagramWebhookEntryChange>>,
    pub field: Option<String>,
    pub value: Option<InstagramWebhookEntryValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstagramWebhook {
    pub object: Option<String>,
    pub entry: Option<Vec<InstagramWebhookEntry>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstagramComment {
    pub id: Option<Uuid>,
    pub instagram_user_id: Option<i64>,
    pub comment_date: Option<DateTime<Utc>>,
    pub comment_from_subscriber_id: Option<String>,
    pub comment_from_subscriber_username: Option<String>,
    pub comment_media_id: Option<String>,
    pub comment_media_product_type: Option<String>,
    pub comment_id: Option<String>,
    pub comment_text: Option<String>,
}
