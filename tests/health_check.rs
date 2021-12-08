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

#[actix_rt::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Setup
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let body = "name=skinny%20pete&email=skinny_pete%40gmail.com";

    // Test
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(200, response.status().as_u16());
}

#[actix_rt::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=skinny%20pete", "missing the email"),
        ("email=skinny_pete%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to send request");

        assert_eq!(
            400,
            response.status().as_u16(),
            // Custom message triggered if the test fails
            "The API did not fail with 400 Bad Request when the payload was {}",
            error_message
        );
    }
}
