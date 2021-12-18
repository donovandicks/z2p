use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use z2p::{
    configuration::get_configuration,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};

/// Runs the web server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("z2p".into(), "info".into());
    init_subscriber(subscriber);

    let config = get_configuration().expect("Failed to read configuration.");
    let conn_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(5))
        .connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("{}:{}", config.application.host, config.application.port);

    let listener = TcpListener::bind(address)?;
    run(listener, conn_pool)?.await
}
