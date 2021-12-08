use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

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
