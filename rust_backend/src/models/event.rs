use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct EventReference {
    id: i32,
    reference: String,

    #[sqlx(rename = "createdat")]
    created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EventReferenceDTO {
    reference: String,

    #[serde(rename = "createdAt")]
    created_at: DateTime<Utc>,
}

impl From<EventReference> for EventReferenceDTO {
    fn from(event: EventReference) -> Self {
        EventReferenceDTO {
            reference: event.reference,
            created_at: event.created_at,
        }
    }
}
