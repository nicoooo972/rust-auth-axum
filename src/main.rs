mod config;
mod handlers;
mod jwt;
mod model;
mod response;
mod route;

use config::Config;
use std::sync::Arc;
use std::time::Duration;
use axum::http::{header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE}, HeaderName, HeaderValue, Method};
use axum::http::header::{ACCESS_CONTROL_ALLOW_CREDENTIALS, COOKIE, SET_COOKIE};
use dotenv::dotenv;
use route::create_router;
use tower_http::cors::CorsLayer;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub struct AppState {
    db: Pool<Postgres>,
    env: Config,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = Config::init();

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE, Method::OPTIONS])
        .allow_credentials(true)
        .allow_headers([
            AUTHORIZATION,
            ACCEPT,
            CONTENT_TYPE,
            COOKIE,
            ACCESS_CONTROL_ALLOW_CREDENTIALS,
        ])
        .expose_headers([SET_COOKIE])
        .max_age(Duration::from_secs(60 * 60));

    let app = create_router(Arc::new(AppState {
        db: pool.clone(),
        env: config.clone(),
    }))
        .layer(cors);

    println!("🚀 Server started successfully");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
