use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};

/// Check the status of the server to see if it is running
///
/// # Returns
///
/// * An `HttpResponse` with status `200` if the server is indeed running
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

/// Builds and returns a server with all routes and middleware
///
/// # Returns
///
/// * `Ok(Server)` if the build process is successful
pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .bind("127.0.0.1:8000")?
        .run();

    Ok(server)
}
