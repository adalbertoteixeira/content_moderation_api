use std::sync::Arc;

use axum::{
    extract::{Json, Path},
    http::{HeaderMap, Method, StatusCode},
    response::{IntoResponse, Response},
    Extension,
};
use serde::{Deserialize, Serialize};
use sqlx::types::{chrono::NaiveDateTime, Uuid};
use tracing::error;
use tracing::info;

use crate::AppState;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClerkUserEmailAddressVerification {
    status: Option<String>,
    strategy: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClerkUserEmailAddress {
    email_address: Option<String>,
    id: Option<String>,
    linked_to: Vec<String>,
    object: String,
    verification: ClerkUserEmailAddressVerification,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClerkUser {
    id: Option<String>,
    username: Option<String>,
    birthday: Option<String>,
    email_addresses: Vec<ClerkUserEmailAddress>,
    first_name: Option<String>,
    gender: Option<String>,
    image_url: Option<String>,
    last_name: Option<String>,
    // created_at: 1654012591514,
    // external_accounts: [],
    // external_id: '567772',
    // last_sign_in_at: 1654012591514,
    // object: 'user',
    // password_enabled: true,
    // phone_numbers: [],
    // primary_phone_number_id: null,
    // primary_web3_wallet_id: null,
    // private_metadata: {},
    // profile_image_url: 'https://www.gravatar.com/avatar?d=mp',
    // public_metadata: {},
    // two_factor_enabled: false,
    // unsafe_metadata: {},
    // updated_at: 1654012591835,
    // web3_wallets: []
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    id: Option<Uuid>,
    clerk_user_id: Option<String>,
    username: Option<String>,
    birthday: Option<NaiveDateTime>,
    email_address: Option<String>,
    first_name: Option<String>,
    gender: Option<String>,
    image_url: Option<String>,
    last_name: Option<String>,
    updated_at: Option<NaiveDateTime>,
}

pub async fn upsert(
    state: Extension<Arc<AppState>>,
    method: Method,
    headers: HeaderMap,
    payload: Json<ClerkUser>,
) -> Response {
    println!("something happened");
    info!("something happened");
    info!("{:?}", payload);
    info!("{:?}", method);
    info!("{:?}", headers);

    let clerk_user_id = payload.id.clone();
    let username = payload.username.clone();
    let first_name = payload.first_name.clone();
    let last_name = payload.last_name.clone();
    let gender = payload.gender.clone();
    let image_url = payload.image_url.clone();
    let email_address = payload.email_addresses[0].email_address.clone();

    let user: Option<User> = match sqlx::query_as!(
        User,
        // language=PostgreSQL
        r#"
            insert into users(
                clerk_user_id,
                username,
                first_name,
                last_name,
                gender,
                image_url,
                email_address,
                updated_at
            )
            values (
                $1, $2, $3, $4, $5, $6, $7, NOW()
            )
            ON CONFLICT (email_address, clerk_user_id) DO UPDATE SET
                username = $2,
                first_name = $3,
                last_name = $4,
                gender = $5,
                image_url = $6,
                updated_at = NOW()
            returning *
        "#,
        clerk_user_id,
        username,
        first_name,
        last_name,
        gender,
        image_url,
        email_address,
    )
    .fetch_one(&state.pool)
    .await
    {
        Ok(user) => Some(user),
        Err(e) => {
            error!("There was an error adding a new user {:?}", e);
            None
        }
    };

    if user.is_some() {
        Json(user.unwrap()).into_response()
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "There was an error saving the user",
        )
            .into_response()
    }
}

pub async fn delete(
    state: Extension<Arc<AppState>>,
    // path: Path<String>,
    Path(clerk_user_id): Path<String>,
    method: Method,
    headers: HeaderMap,
) -> Response {
    println!("something happened");
    info!("something happened");
    info!("{:?}", method);
    // info!("Path {:?}", path);
    info!("{:?}", headers);
    // let clerk_user_id = path.
    info!("id: {:?}", clerk_user_id);

    // let clerk_user_id = "".to_owned();

    let user_deleted: bool = match sqlx::query!(
        // language=PostgreSQL
        r#"DELETE FROM users where clerk_user_id = $1;"#,
        clerk_user_id,
    )
    .execute(&state.pool)
    .await
    {
        Ok(e) => {
            info!("DELETED {:?}", e);
            true
        }
        Err(e) => {
            error!("There was an error deleting the user {:?}", e);
            false
        }
    };

    info!("DEBUG deletion {:?}", user_deleted);
    if user_deleted == true {
        (StatusCode::NO_CONTENT, "User deleted successfully").into_response()
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "There was an error deleteing the user",
        )
            .into_response()
    }
}
