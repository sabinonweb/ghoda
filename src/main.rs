use actix_web::{App, HttpServer};
use ghoda::startup::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    run(listener)?.await
}
