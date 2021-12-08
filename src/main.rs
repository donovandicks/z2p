use z2p::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run().await
}
