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
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let req_id = Uuid::new_v4();
    let req_span = tracing::info_span!(
        "Adding a new subscriber",
        request_id = %req_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name,
    );

    let _req_span_guard = req_span.enter();

    tracing::info!(
        "request_id {} - Saving new subscriber details in the database",
        req_id,
    );

    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => {
            tracing::info!(
                "request_id {} - New subscriber details have been saved",
                req_id,
            );
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!("request_id {} - Failed to execute query: {:?}", req_id, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
