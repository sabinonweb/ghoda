use crate::routes::{health_check::health_check, subscriptions::subscribe};
use actix_web::{dev::Server, middleware::Logger, web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, connection: PgPool) -> std::io::Result<Server> {
    let connection = web::Data::new(connection);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(connection.clone())
        // .app_data(connection)
    })
    .listen(listener)?
    .run();
    Ok(server)
}
