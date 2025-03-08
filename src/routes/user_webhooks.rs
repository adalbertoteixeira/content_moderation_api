use std::sync::Arc;

use crate::types;
use axum::{
    extract::{Json, Path},
    http::{HeaderMap, Method, StatusCode},
    response::{IntoResponse, Response},
    Extension,
};
use tracing::error;
use tracing::info;

use crate::AppState;

pub async fn upsert(
    state: Extension<Arc<AppState>>,
    method: Method,
    headers: HeaderMap,
    payload: Json<types::clerk::ClerkUser>,
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

    let user: Option<types::user::User> = match sqlx::query_as!(
        types::user::User,
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
            ON CONFLICT ( clerk_user_id) DO UPDATE SET
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
