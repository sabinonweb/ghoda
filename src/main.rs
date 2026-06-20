use ghoda::{
    configurations::get_configurations,
    email_client::EmailClient,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configurations = get_configurations().expect("Failed to read configurations!");
    let connection = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy(&configurations.database.connection_string())
        .expect("Failed to create Postgres connection pool.");

    let sender_email = configurations
        .email_client
        .sender()
        .expect("Invalid sender email address");
    let email_client = EmailClient::new(configurations.email_client.base_url, sender_email);

    let address = format!(
        "{}:{}",
        configurations.application.host, configurations.application.port
    );
    let listener = TcpListener::bind(address.clone()).unwrap();
    run(listener, connection, email_client)?.await
}
