use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};
use mongodb::{
    bson::{doc, to_document, Document},
    Client, Collection,
};
use serde_json::Value;
use sqlx::PgPool;

use crate::models::event::{EventReference, EventReferenceDTO};

#[derive(Clone)]
pub struct Service {
    pub pg_pool: PgPool,
    pub mongo_client: Arc<Client>,
}

pub async fn get_events(service: web::Data<Service>) -> impl Responder {
    let pg_pool = &service.as_ref().pg_pool;
    let items =
        sqlx::query_as::<_, EventReference>("SELECT * FROM event ORDER BY id DESC LIMIT 500")
            .fetch_all(pg_pool)
            .await;

    match items {
        Ok(items) => {
            let dto_items: Vec<EventReferenceDTO> =
                items.into_iter().map(|item| item.into()).collect();
            HttpResponse::Ok().json(dto_items)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn handle_event(service: web::Data<Service>, event: web::Json<Value>) -> impl Responder {
    let pg_pool = &service.as_ref().pg_pool;
    let mongodb = &service.as_ref().mongo_client.clone();

    let event_schema = crate::detect_event_schema(&event);
    let collection: Collection<Document> = mongodb.database("events_db").collection("events");

    let event_document = match to_document(&event) {
        Ok(doc) => doc,
        Err(err) => {
            return HttpResponse::BadRequest().body(format!("Invalid event data: {}", err));
        }
    };

    let new_event = doc! {
        "schemaId": &event_schema,
        "metadata": &event_document,
    };

    if let Err(err) = collection.insert_one(new_event).await {
        println!("Failed to store event: {}", err);
    }

    // Store event reference in PostgreSQL
    let query = "INSERT INTO event (reference) VALUES ($1) ON CONFLICT (reference) DO NOTHING";
    if let Err(err) = sqlx::query(query).bind(event_schema).execute(pg_pool).await {
        return HttpResponse::InternalServerError()
            .body(format!("Failed to store event reference: {}", err));
    }

    HttpResponse::Ok().finish()
}

pub async fn get_event(service: web::Data<Service>, path: web::Path<String>) -> impl Responder {
    let mongodb = &service.as_ref().mongo_client.clone();
    let schema_id = path.into_inner();
    let collection: Collection<Document> = mongodb.database("events_db").collection("events");

    let filter = doc! { "schemaId": schema_id };
    let event_result = collection.find_one(filter).await;

    match event_result {
        Ok(Some(mut event)) => {
            // Doing some swapping around since mongo gives us some extra garbage
            event.remove("_id");
            if let Some(metadata) = event.remove("metadata") {
                event.insert("event", metadata);
            }

            HttpResponse::Ok().json(event)
        }
        Ok(None) => HttpResponse::NotFound().body("Event not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    }
}
