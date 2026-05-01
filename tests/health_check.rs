use ghoda::run;

#[tokio::test]
async fn health_check_test() {
    spawn_app();

    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:8080/health_check")
        .send()
        .await
        .expect("Failed to send the GET request");

    println!("response: {:?}", response.status());
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    tokio::spawn(ghoda::run().expect("Failed to get server"));
    // let server = run().expect("Failed to get the server");
    // let _ = tokio::spawn(server);
}
