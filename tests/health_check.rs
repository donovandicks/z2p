use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;
use z2p::configuration::{get_configuration, DatabaseSettings};

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

/// Spawns the app as a background process on a random port so it can be used
/// for testing
///
/// # Returns
///
/// * The address on which the app is listening
async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut config = get_configuration().expect("Failed to read configuration");
    config.database.database_name = Uuid::new_v4().to_string();

    let conn_pool = configure_database(&config.database).await;

    let server = z2p::startup::run(listener, conn_pool.clone()).expect("Failed to bind to address");

    // Spawn the server as a background task, allowing it to run concurrently
    // with downstream futures and tasks, including tests
    let _ = tokio::spawn(server);

    TestApp {
        address,
        db_pool: conn_pool,
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut conn = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres.");

    conn.execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    let conn_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&conn_pool)
        .await
        .expect("Failed to migrate the databse.");

    conn_pool
}

#[actix_rt::test]
async fn health_check_works() {
    // Setup
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Test
    let response = client
        .get(format!("{}/health_check", app.address))
        .send()
        .await
        .expect("Failed to send request");

    assert!(response.status().is_success()); // 200 status
    assert_eq!(Some(0), response.content_length()); // No content
}

#[actix_rt::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Setup
    let app = spawn_app().await;

    let client = reqwest::Client::new();
    let body = "name=skinny%20pete&email=skinny_pete%40gmail.com";

    // Test
    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq!(saved.email, "skinny_pete@gmail.com");
    assert_eq!(saved.name, "skinny pete");
}

#[actix_rt::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=skinny%20pete", "missing the email"),
        ("email=skinny_pete%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
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
