use actix_web::HttpResponse;

/// Check the status of the server to see if it is running
///
/// # Returns
///
/// * An `HttpResponse` with status `200` if the server is indeed running
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
