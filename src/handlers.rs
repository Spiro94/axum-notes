use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::AppError,
    models::{CreateNote, Note, UpdateNote},
};

// GET /health
#[utoipa::path(get, path = "/health", responses((status=200, description= "Health Check")))]
pub async fn health() -> &'static str {
    "ok"
}

// GET /notes
#[utoipa::path(get, path = "/notes", responses((status = 200, description = "List of notes", body = Vec<Note>), (status = 500, description = "Internal server error")))]
pub async fn list_notes(State(pool): State<PgPool>) -> Result<Json<Vec<Note>>, AppError> {
    let notes = sqlx::query_as!(
        Note,
        "SELECT id, title, body, created_at, updated_at
         FROM notes
         ORDER BY created_at DESC"
    )
    .fetch_all(&pool)
    .await?;
    Ok(Json(notes))
}

// POST /notes
#[utoipa::path(post, path = "/notes", request_body = CreateNote, responses((status = 201, description = "Note created", body = Note), (status = 400, description = "Bad request"), (status = 500, description = "Internal server error")))]
pub async fn create_note(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateNote>,
) -> Result<(StatusCode, Json<Note>), AppError> {
    if payload.title.trim().is_empty() {
        return Err(AppError::BadRequest("title must not be empty".to_string()));
    }

    let note = sqlx::query_as!(
        Note,
        "INSERT INTO notes(title, body)
    VALUES ($1, $2)
    RETURNING id, title, body, created_at, updated_at",
        payload.title,
        payload.body,
    )
    .fetch_one(&pool)
    .await?;

    Ok((StatusCode::CREATED, Json(note)))
}

// GET /notes/:id
#[utoipa::path(get, path = "/notes/{id}", responses((status = 200, description = "Note found", body = Note), (status = 404, description = "Note not found"), (status = 500, description = "Internal server error")))]
pub async fn get_note(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Note>, AppError> {
    let note = sqlx::query_as!(
        Note,
        "SELECT id, title, body, created_at, updated_at
    FROM notes 
    WHERE id=$1",
        id
    )
    .fetch_optional(&pool)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(Json(note))
}

// PUT /notes/:id
#[utoipa::path(put, path = "/notes/{id}", request_body = UpdateNote, responses((status = 200, description = "Note updated", body = Note), (status = 404, description = "Note not found"), (status = 500, description = "Internal server error")))]
pub async fn update_note(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateNote>,
) -> Result<(StatusCode, Json<Note>), AppError> {
    let existing = sqlx::query_as!(
        Note,
        "SELECT id, title, body, created_at, updated_at
    FROM notes 
    WHERE id=$1",
        id
    )
    .fetch_optional(&pool)
    .await?
    .ok_or(AppError::NotFound)?;

    let new_title = payload.title.unwrap_or(existing.title);
    let new_body = payload.body.unwrap_or(existing.body);

    let updated_note = sqlx::query_as!(
        Note,
        "UPDATE notes
    SET title = $1, body = $2, updated_at = NOW()
    WHERE id = $3
    RETURNING id, title, body, created_at, updated_at",
        new_title,
        new_body,
        id
    )
    .fetch_one(&pool)
    .await?;

    Ok((StatusCode::OK, Json(updated_note)))
}

// DELETE /notes/:id
#[utoipa::path(delete, path = "/notes/{id}", responses((status = 200, description = "Note deleted"), (status = 404, description = "Note not found"), (status = 500, description = "Internal server error")))]
pub async fn delete_note(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let result = sqlx::query!(
        "DELETE FROM notes
    WHERE id = $1",
        id
    )
    .execute(&pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(StatusCode::OK)
}
