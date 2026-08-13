use crate::{
    configurations::Settings,
    email_client::EmailClient,
    routes::{health_check::health_check, subscriptions::subscribe},
};
use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configurations: Settings) -> Result<Self, std::io::Error> {
        let connection = get_connection_pool(&configurations);

        let sender_email = configurations
            .email_client
            .sender()
            .expect("Invalid sender email address");
        let timeout = configurations.email_client.timeout();
        let email_client = EmailClient::new(
            configurations.email_client.base_url,
            sender_email,
            configurations.email_client.authorization_token,
            timeout,
        );

        let address = format!(
            "{}:{}",
            configurations.application.host, configurations.application.port
        );
        println!("Address: {:?}", address);
        let listener = TcpListener::bind(address.clone()).unwrap();
        let port = listener.local_addr().unwrap().port();
        let server = run(listener, connection, email_client)?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn run(
    listener: TcpListener,
    connection: PgPool,
    email_client: EmailClient,
) -> std::io::Result<Server> {
    let connection = web::Data::new(connection);
    let email_client = web::Data::new(email_client);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(connection.clone())
            .app_data(email_client.clone())
        // .app_data(connection)
    })
    .listen(listener)?
    .run();
    Ok(server)
}

pub fn get_connection_pool(configurations: &Settings) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy(&configurations.database.connection_string())
        .expect("Failed to create Postgres connection pool.")
}
