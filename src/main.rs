use env_logger::Env;
use ghoda::{configurations::get_configurations, startup::run};
use sqlx::{Connection, PgPool};
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let configurations = get_configurations().expect("Failed to read configurations!");
    let connection = PgPool::connect(&configurations.database.connection_string())
        .await
        .expect("Failed a connection to postgres");
    let address = format!("127.0.0.1:{}", configurations.application_port);
    let listener = TcpListener::bind(address.clone()).unwrap();
    println!("Listening on: {:?}", address);
    run(listener, connection)?.await
}
