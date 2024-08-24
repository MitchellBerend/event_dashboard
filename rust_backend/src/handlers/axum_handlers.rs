use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json
};

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

pub async fn get_events(
    State(service): State<Service>,
) -> Result<Json<Vec<EventReferenceDTO>>, StatusCode> {
    let items =
        sqlx::query_as::<_, EventReference>("SELECT * FROM event ORDER BY id DESC LIMIT 500")
            .fetch_all(&service.pg_pool)
            .await;

    match items {
        Ok(items) => {
            let dto_items: Vec<EventReferenceDTO> =
                items.into_iter().map(|item| item.into()).collect();
            Ok(Json(dto_items))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn handle_event(
    State(service): State<Service>,
    Json(event): Json<Value>,
) -> (StatusCode, String) {
    let event_schema = crate::detect_event_schema(&event);
    let collection: Collection<Document> = service
        .mongo_client
        .database("events_db")
        .collection("events");

    let event_document = match to_document(&event) {
        Ok(doc) => doc,
        Err(err) => {
            return (
                StatusCode::BAD_REQUEST,
                format!("Invalid event data: {}", err),
            );
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
    if let Err(err) = sqlx::query(query)
        .bind(event_schema)
        .execute(&service.pg_pool)
        .await
    {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to store event reference: {}", err),
        );
    }

    (StatusCode::OK, "Event stored".to_string())
}

pub async fn get_event(
    State(service): State<Service>,
    Query(schema_id): Query<String>,
) -> Result<Json<Document>, (StatusCode, String)> {
    let collection: Collection<Document> = service
        .mongo_client
        .database("events_db")
        .collection("events");

    let filter = doc! { "schemaId": schema_id };
    let event_result = collection.find_one(filter).await;

    match event_result {
        Ok(Some(mut event)) => {
            // Doing some swapping around since mongo gives us some extra garbage
            event.remove("_id");
            if let Some(metadata) = event.remove("metadata") {
                event.insert("event", metadata);
            }

            Ok(Json(event))
        }
        Ok(None) => Err((StatusCode::NOT_FOUND, "Event not found".to_string())),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get event: {}", e),
        )),
    }
}
