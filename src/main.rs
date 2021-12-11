use env_logger::Env;
use sqlx::PgPool;
use std::net::TcpListener;
use z2p::{configuration::get_configuration, startup::run};

/// Runs the web server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Fallback to >= info level if the RUST_LOG variable has not been set
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let config = get_configuration().expect("Failed to read configuration.");
    let conn_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", config.application_port);

    let listener = TcpListener::bind(address)?;
    run(listener, conn_pool)?.await
}
