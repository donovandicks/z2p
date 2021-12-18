#![allow(clippy::async_yields_async)]
use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use tracing;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

/// Inserts a subscriber into the database
///
/// # Args
///
/// * `pool` - A connection pool to the Postgres database
/// * `form` - The form data submitted by the user when they subscribe
///
/// # Returns
///
/// * `()` On success
///
/// # Errors
///
/// * `sqlx::Error` If an error is encountered when inserting data into the
///     database
pub async fn insert_subscriber(pool: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(())
}

/// Subscribes a user to the newsletter
///
/// # Args
///
/// * `form` - The `FormData` entered by the user
/// * `conn` - The database connection
///
/// # Returns
///
/// * An `HttpResponse` reflecting the status of the operation
#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pool),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name,
    )
)]
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    match insert_subscriber(&pool, &form).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
