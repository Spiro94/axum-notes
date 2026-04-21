use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

use utoipa::OpenApi;
#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct Note {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateNote {
    pub title: String,
    pub body: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateNote {
    pub title: Option<String>,
    pub body: Option<String>,
}

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::health,
        crate::handlers::list_notes,
        crate::handlers::create_note,
        crate::handlers::get_note,
        crate::handlers::update_note,
        crate::handlers::delete_note,
    ),
    components(
        schemas(Note, CreateNote, UpdateNote)
    ),
    tags(
        (name = "notes", description = "Notes management endpoints")
    ),
    info(
        title = "Notes API",
        version = "1.0.0",
        description = "REST API for managing notes with PostgreSQL backend"
    )
)]
pub struct ApiDoc;
