use ghoda::{configurations::get_configurations, startup::run};
use sqlx::{Connection, PgPool};
use std::net::TcpListener;
use tracing::dispatcher::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new("ghoda".into(), std::io::stdout);
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
        .into();

    // To define which subscriber should be used to process the spans
    set_global_default(subscriber).expect("Failed to get a subscriber");

    let configurations = get_configurations().expect("Failed to read configurations!");
    let connection = PgPool::connect(&configurations.database.connection_string())
        .await
        .expect("Failed a connection to postgres");
    let address = format!("127.0.0.1:{}", configurations.application_port);
    let listener = TcpListener::bind(address.clone()).unwrap();
    println!("Listening on: {:?}", address);
    run(listener, connection)?.await
}
