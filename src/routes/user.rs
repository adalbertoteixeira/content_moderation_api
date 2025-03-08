use std::sync::Arc;

use axum::{
    extract::Request,
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use clerk_rs::validators::authorizer::ClerkJwt;
use serde::{Deserialize, Serialize};
use tracing::{error, info};

use crate::AppState;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Whoami {
    clerk_user_id: Option<String>,
    instagram_usernames: Option<Vec<String>>,
}
#[derive(Serialize)]
pub struct User {
    id: Option<String>,
    username: Option<String>,
    clerk_user_id: Option<String>,
    instagram_usernames: Option<Vec<String>>,
}
pub async fn whoami(state: Extension<Arc<AppState>>, req: Request) -> Response {
    // -> Json<User> {
    let extensions = req.extensions();
    // let state = req
    info!("extensions: {:?}", extensions.get::<ClerkJwt>());
    info!("State: {:?}", &state.pool);
    if extensions.get::<ClerkJwt>().is_none() {
        return (StatusCode::INTERNAL_SERVER_ERROR, "There was an error").into_response();
    }
    let claims = extensions.get::<ClerkJwt>().unwrap();
    println!("Claims: {:?}", claims);
    let clerk_user_id = &claims.sub;
    info!("User: {:?}", clerk_user_id);
    let user: Option<Whoami> = match sqlx::query_as!(
        Whoami,
        // language=PostgreSQL
        r#"
            SELECT
                    users.clerk_user_id,
                    array_remove(ARRAY_AGG(instagram.username), null) AS instagram_usernames
            FROM
                    users
                    LEFT JOIN instagram ON instagram.clerk_user_id = users.clerk_user_id
            WHERE
                    users.clerk_user_id = $1
            GROUP BY
                    users.clerk_user_id;
        "#,
        clerk_user_id,
    )
    .fetch_one(&state.pool)
    .await
    {
        Ok(user) => Some(user),
        Err(e) => {
            error!("There was an error finding the user {:?}", e);
            None
        }
    };

    if user.is_some() {
        Json(user.unwrap()).into_response()
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, "There was an error").into_response()
    }
}
