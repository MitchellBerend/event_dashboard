use std::env;
use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client};
use sqlx::PgPool;
use tower_http::cors::CorsLayer;

use rust_backend::handlers::axum_handlers;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    dotenv().ok();

    // Postgres connection
    let pool = {
        let database_uri = env::var("POSTGRES_URI").expect("POSTGRES_URI must be set");
        PgPool::connect(&database_uri)
            .await
            .expect("Failed to create pool.")
    };

    // Mongo connection
    let mongo_client = {
        let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI must be set");
        let mongo_client_options = ClientOptions::parse(mongo_uri).await.unwrap();
        Client::with_options(mongo_client_options).unwrap()
    };

    sqlx::migrate!().run(&pool).await.unwrap();

    let service = axum_handlers::Service {
        pg_pool: pool.clone(),
        mongo_client: Arc::new(mongo_client),
    };

    let app = Router::new()
        .route("/events", get(axum_handlers::get_events))
        .route("/event/:schema_id", get(axum_handlers::get_event))
        .route("/event", post(axum_handlers::handle_event))
        .with_state(service)
        .layer(CorsLayer::permissive());

    // run our app with hyper, listening globally on port 8081
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8081").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
