use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: Option<Uuid>,
    pub clerk_user_id: Option<String>,
    pub username: Option<String>,
    pub birthday: Option<DateTime<Utc>>,
    pub email_address: Option<String>,
    pub first_name: Option<String>,
    pub gender: Option<String>,
    pub image_url: Option<String>,
    pub last_name: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
