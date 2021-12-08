use actix_web::{web, App, HttpResponse, HttpServer, Responder};

/// Check the status of the server to see if it is running
///
/// # Returns
///
/// * An `HttpResponse` with status `200` if the server is indeed running
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
