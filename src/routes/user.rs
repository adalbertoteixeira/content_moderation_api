use axum::{extract::Request, Json};
use serde::Serialize;
use tracing::info;

#[derive(Serialize)]
pub struct User {
    id: Option<String>,
    username: Option<String>,
}
pub async fn whoami(req: Request) -> Json<User> {
    info!("something happened");
    info!("Request: {:?}", req);
    use clerk_rs::validators::authorizer::ClerkJwt;
    let extensions = req.extensions();
    info!("extensions: {:?}", extensions.get::<ClerkJwt>());
    if extensions.get::<ClerkJwt>().is_some() {
        let claims = extensions.get::<ClerkJwt>().unwrap();
        println!("Claims: {:?}", claims);
    }
    // Html("Who am i")
    let user = User {
        id: None,
        username: None,
    };
    Json(user)
}
