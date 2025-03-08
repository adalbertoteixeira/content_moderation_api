use std::{sync::Arc, time::Duration};

use crate::types;
use crate::types::instagram::InstagramComment;
use crate::AppState;
use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension,
};
use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use tracing::error;
use tracing::info;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstagramAccount {
    pub id: Option<Uuid>,
    pub username: Option<String>,
    pub instagram_user_id: Option<i64>,
    pub instagram_access_token: Option<String>,
}
async fn create_instagram_comment(
    instagram_user_id: i64,
    comment_date: DateTime<Utc>,
    comment_id: String,
    comment_text: String,
    comment_media_id: String,
    comment_from_subscriber_id: String,
    comment_from_subscriber_username: String,
    comment_media_product_type: String,
    state: &Extension<Arc<AppState>>,
) -> Option<InstagramComment> {
    let created_instagram_comment: Option<InstagramComment> = match sqlx::query_as!(
        InstagramComment,
        r#"
            INSERT INTO instagram_comments (
               instagram_user_id,
               comment_text,
               comment_id,
               comment_media_id,
               comment_from_subscriber_id,
               comment_from_subscriber_username,
               comment_media_product_type,
               comment_date
            ) VALUES (
            $1,$2, $3, $4, $5, $6,$7, $8
            ) RETURNING *;
        "#,
        instagram_user_id,
        comment_text,
        comment_id,
        comment_media_id,
        comment_from_subscriber_id,
        comment_from_subscriber_username,
        comment_media_product_type,
        comment_date
    )
    .fetch_one(&state.pool)
    .await
    {
        Ok(result) => Some(result),
        Err(e) => {
            error!(
                "There was an error saving the comment to the database: {:?}",
                e
            );
            None
        }
    };
    created_instagram_comment
}

async fn get_instagram_account(
    instagram_user_id: i64,
    state: &Extension<Arc<AppState>>,
) -> Option<InstagramAccount> {
    info!("entry: {:?}", instagram_user_id);
    let existing_instagram_entry: Option<InstagramAccount> = match sqlx::query_as!(
        InstagramAccount,
        r#"
            SELECT
                id, instagram_user_id, instagram_access_token, username
            FROM
                instagram
            WHERE
                instagram_user_id = $1;
        "#,
        instagram_user_id,
    )
    .fetch_one(&state.pool)
    .await
    {
        Ok(result) => Some(result),
        Err(e) => {
            error!(
                "There was an error trying to find the user for the webhook {:?}",
                e
            );
            None
        }
    };
    existing_instagram_entry
}

pub async fn upsert_webhooks(
    state: Extension<Arc<AppState>>,
    payload: Json<types::instagram::InstagramWebhook>,
) -> Response {
    info!("payload: {:?}", payload);
    let payload_object = payload.object.clone();
    if payload_object.is_none_or(|x| &x != "instagram") {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "We can't parse the payload. Unknown object type",
        )
            .into_response();
    }

    let payload_entry = payload.entry.clone();
    if payload_entry.as_ref().is_none_or(|x| x.is_empty()) {
        return (StatusCode::ACCEPTED, "No entries found in the payload").into_response();
    }

    for entry in payload_entry.unwrap() {
        if entry.id.is_none() {
            break;
        }
        let entry_instagram_id = entry.id.clone().unwrap().parse::<i64>().unwrap();
        let existing_instagram_entry =
            get_instagram_account(entry_instagram_id.clone(), &state).await;
        info!("existing_instagram_entry: {:?}", existing_instagram_entry);
        if existing_instagram_entry.is_none() {
            break;
        }

        let payload_changes = entry.changes.clone();
        if payload_changes.as_ref().is_none_or(|x| x.is_empty()) {
            info!("No payload_changes: {:?}", payload_changes);
            break;
        }
        info!("payload_changes: {:?}", payload_changes);
        for change in payload_changes.unwrap() {
            info!("change: {:?}", change);
            if change.field.as_ref().is_none_or(|x| x != "comments") {
                info!("Change is not a comment");
                break;
            }
            let payload_timestamp = entry.time.clone().unwrap();
            let dt_t = DateTime::from_timestamp(payload_timestamp, 0).unwrap();
            let payload_media = change.value.clone().unwrap().media.unwrap();
            let payload_from = change.value.clone().unwrap().from.unwrap();
            let _ = create_instagram_comment(
                entry_instagram_id.clone(),
                dt_t,
                change.value.clone().unwrap().id.unwrap(),
                change.value.clone().unwrap().text.unwrap(),
                payload_media.id.unwrap(),
                payload_from.id.unwrap(),
                payload_from.username.unwrap(),
                payload_media.media_product_type.unwrap(),
                &state,
            )
            .await;
        }
    }

    (StatusCode::ACCEPTED,).into_response()
}
pub async fn upsert_instagram_access_token(
    state: Extension<Arc<AppState>>,
    payload: Json<types::instagram::InstagramAccessToken>,
) -> Response {
    let instagram_user_id = payload.instagram_user_id.clone();
    let clerk_user_id = payload.clerk_user_id.clone();
    let instagram_access_token = payload.access_token.clone();
    let expires_in = payload.expires_in.clone();
    let now = Utc::now();
    let duration = Duration::from_secs(expires_in.unwrap().try_into().unwrap());
    let instagram_access_token_expires_at = now + duration;

    let url = reqwest::Url::parse_with_params(
        "https://graph.instagram.com/v22.0/me",
        &[
            ("fields", "user_id,username,profile_picture_url"),
            ("access_token", &instagram_access_token.clone().unwrap()),
        ],
    )
    .unwrap();
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .send()
        .await
        .unwrap()
        .json::<types::instagram::InstagramMe>()
        .await
        .unwrap();
    let instagram: Option<types::instagram::Instagram> = match sqlx::query_as!(
        types::instagram::Instagram,
        // language=PostgreSQL
        r#"
            insert into instagram(
                username,
                instagram_user_id,
                instagram_access_token,
                instagram_access_token_expires_at,
                clerk_user_id,
                profile_picture_url
            )
            values (
            $1, $2, $3, NOW()::timestamp + INTERVAL '$4 seconds', $5, $6
            )
            ON CONFLICT ( clerk_user_id) DO UPDATE SET

            username = $1, instagram_user_id=$2, instagram_access_token=$3, instagram_access_token_expires_at=$4
            returning *
        "#,
        response.username,
        instagram_user_id,
        instagram_access_token,
        instagram_access_token_expires_at,
        clerk_user_id,
        response.profile_picture_url

    )
    .fetch_one(&state.pool)
    .await
    {
        Ok(result) => Some(result),
        Err(e) => {
            error!("There was an error adding a new instagram user {:?}", e);
            None
        }
    };

    if instagram.is_some() {
        Json(instagram.unwrap()).into_response()
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "There was an error saving the datae",
        )
            .into_response()
    }
}
