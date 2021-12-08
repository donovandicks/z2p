use z2p::run;

/// Runs the web server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run()?.await
}
