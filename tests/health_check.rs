use std::net::TcpListener;

/// Spawns the app as a background process on a random port so it can be used
/// for testing
///
/// # Returns
///
/// * The address on which the app is listening
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();

    let server = z2p::run(listener).expect("Failed to bind to address");

    // Spawn the server as a background task, allowing it to run concurrently
    // with downstream futures and tasks, including tests
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

#[actix_rt::test]
async fn health_check_works() {
    // Setup
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Test
    let response = client
        .get(format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to send request");

    assert!(response.status().is_success()); // 200 status
    assert_eq!(Some(0), response.content_length()); // No content
}
