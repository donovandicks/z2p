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
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .listen(listener)?
        .run();

    Ok(server)
}
