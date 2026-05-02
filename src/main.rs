use ghoda::{configurations::get_configurations, startup::run};
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configurations = get_configurations().expect("Failed to read configurations!");
    let address = format!("127.0.0.1:{}", configurations.application_port);
    let listener = TcpListener::bind(address).unwrap();
    run(listener)?.await
}
