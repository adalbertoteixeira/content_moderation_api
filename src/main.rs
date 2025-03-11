use std::sync::Arc;

use axum::{
    routing::{delete, get, post},
    Extension, Router,
};
use clerk_rs::{
    clerk::Clerk,
    validators::{axum::ClerkLayer, jwks::MemoryCacheJwksProvider},
    ClerkConfiguration,
};
use dotenvy::dotenv;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tracing::{error, info};
mod routes;
mod types;

struct AppState {
    pool: PgPool,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    dotenv().ok();
    info!("Starting server...");
    let clerk_secret_key = std::env::var("CLERK_SECRET_KEY").expect("CLERK_SECRET_KEY must be set");
    let content_moderation_database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    info!("DATABASE_URL is {}", content_moderation_database_url);
    // Database connection
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&content_moderation_database_url)
        .await
    {
        Ok(pool) => {
            info!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            error!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let _ = match sqlx::migrate!("./migrations").run(&pool).await {
        Ok(()) => info!("�� Migrations executed successfully"),
        Err(err) => {
            error!("�� Failed to execute migrations: {:?}", err);
            std::process::exit(1);
        }
    };
    // Clerk
    let config = ClerkConfiguration::new(None, None, Some(clerk_secret_key.to_string()), None);
    let clerk = Clerk::new(config);
    let shared_state = Arc::new(AppState { pool: pool.clone() });
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/webhooks/user", post(routes::user_webhooks::upsert))
        .route(
            "/webhooks/user/{clerk_user_id}",
            delete(routes::user_webhooks::delete),
        )
        .route(
            "/instagram/upsert_instagram_access_token",
            post(routes::instagram::upsert_instagram_access_token),
        )
        .route(
            "/instagram/webhook_upsert",
            post(routes::instagram::upsert_webhooks),
        )
        .route("/whoami", get(routes::user::whoami))
        .layer(ClerkLayer::new(
            MemoryCacheJwksProvider::new(clerk.clone()),
            Some(vec![
                "/instagram/upsert_instagram_access_token".to_string(),
                "/whoami".to_string(),
            ]),
            true,
        ))
        .layer(Extension(shared_state));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
