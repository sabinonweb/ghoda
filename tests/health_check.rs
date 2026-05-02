use std::net::TcpListener;

use ghoda::configurations::get_configurations;
use sqlx::{Connection, PgConnection};

#[tokio::test]
async fn health_check_test() {
    let address = spawn_app();

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to send the GET request");

    println!("response: {:?}", response.status());
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    tokio::spawn(ghoda::startup::run(listener).expect("Failed to get server"));
    format!("http://127.0.0.1:{}", port)
    // let server = run().expect("Failed to get the server");
    // let _ = tokio::spawn(server);
}

#[tokio::test]
async fn subcribe_returns_a_200_for_valid_request_data() {
    let address = spawn_app();
    let configuations = get_configurations().expect("Failed to get the configurations!");
    let connection_string = configuations.database.connection_string();
    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres");
    let client = reqwest::Client::new();

    println!("Address: {:?}", address);
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status().as_u16(), 200);

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscriptions");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subcribe_returns_a_400_for_invalid_request_data() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
