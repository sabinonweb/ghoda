use ghoda::{
    configurations::get_configurations,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configurations = get_configurations().expect("Failed to read configurations!");
    let connection = PgPool::connect(&configurations.database.connection_string())
        .await
        .expect("Failed a connection to postgres");
    let address = format!("127.0.0.1:{}", configurations.application_port);
    let listener = TcpListener::bind(address.clone()).unwrap();
    run(listener, connection)?.await
}
