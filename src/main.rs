use std::net::TcpListener;
use z2p::{configuration::get_configuration, startup::run};

/// Runs the web server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = get_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", config.application_port);

    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
