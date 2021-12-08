fn spawn_app() {
    let server = z2p::run().expect("Failed to bind to address");

    // Spawn the server as a background task, allowing it to run concurrently
    // with downstream futures and tasks, including tests
    let _ = tokio::spawn(server);
}

#[actix_rt::test]
async fn health_check_works() {
    // Setup
    spawn_app();
    let client = reqwest::Client::new();

    // Test
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to send request");

    assert!(response.status().is_success()); // 200 status
    assert_eq!(Some(0), response.content_length()); // No content
}
