use std::net::TcpListener;
use z2p::startup::run;

/// Runs the web server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind to address");
    run(listener)?.await
}
