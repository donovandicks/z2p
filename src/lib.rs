use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use std::net::TcpListener;

/// Check the status of the server to see if it is running
///
/// # Returns
///
/// * An `HttpResponse` with status `200` if the server is indeed running
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

/// Builds and returns a server with all routes and middleware, listening on the
/// given listener
///
/// # Args
///
/// * `listener` - A `TcpListener` that is already bound to an address
///
/// # Returns
///
/// * `Ok(Server)` if the build process is successful
///
/// # Errors
///
/// * Can occur if the server fails to listen on the given listener
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
