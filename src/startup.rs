use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

/// Builds and returns a server with all routes and middleware, listening on the
/// given listener
///
/// # Args
///
/// * `listener` - A `TcpListener` that is already bound to an address
/// * `connection` - A `PgConnection` that represents a connection to a Postgres
///     database instance
///
/// # Returns
///
/// * `Ok(Server)` if the build process is successful
///
/// # Errors
///
/// * Can occur if the server fails to listen on the given listener
pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    // Wrap the connection in an `Arc`
    let connection = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
