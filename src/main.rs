use actix_web::{App, HttpServer};
use ghoda::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run()?.await
}
