use std::env;
use std::sync::Arc;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http, web, App, HttpServer};
use dotenv::dotenv;
use env_logger::Env;
use mongodb::{options::ClientOptions, Client};
use sqlx::PgPool;

use rust_backend::handlers::actix_handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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

    // Logging setup
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    sqlx::migrate!().run(&pool).await.unwrap();

    let service = actix_handlers::Service {
        pg_pool: pool.clone(),
        mongo_client: Arc::new(mongo_client),
    };

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
                http::header::CONTENT_TYPE,
                http::header::ORIGIN,
            ])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(web::Data::new(service.clone()))
            .route("/events", web::get().to(actix_handlers::get_events))
            .route("/event/{id}", web::get().to(actix_handlers::get_event))
            .route("/event", web::post().to(actix_handlers::handle_event))
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}
