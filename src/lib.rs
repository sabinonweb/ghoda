use actix_web::{dev::Server, get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hello world")
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("world");
    format!("hello {}", name)
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub fn run() -> std::io::Result<Server> {
    Ok(HttpServer::new(|| {
        App::new()
            .service(hello)
            .route("/greet", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
    })
    .bind(("127.0.0.1", 8080))?
    .run())
}
