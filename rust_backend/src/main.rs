use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::env;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
struct EventReference {
    id: i32,
    reference: String,
}

async fn get_events(pool: web::Data<PgPool>) -> impl Responder {
    let items = sqlx::query_as::<_, EventReference>(
        "SELECT id, reference FROM event ORDER BY id DESC LIMIT 500",
    )
    .fetch_all(pool.get_ref())
    .await;

    match items {
        Ok(items) => {
            println!("Sending {:?}", &items);
            HttpResponse::Ok().json(items)
        }
        Err(e) => {
            println!("Error: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

async fn hello(pool: web::Data<PgPool>) -> impl Responder {
    let query = sqlx::query_as::<_, EventReference>("SELECT * FROM event")
        .fetch_all(pool.get_ref())
        .await
        .unwrap();
    HttpResponse::Ok().body(format!("{:?}", query))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("POSTGRES_URI").expect("POSTGRES_URI must be set");
    println!("{}", database_url);

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to create pool.");

    println!("listening on 127.0.0.1:8081");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/events", web::get().to(get_events))
            .route("/", web::get().to(hello))
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}
