// src/main.rs
use actix_web::{web, App, HttpServer, HttpResponse, Responder};

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

async fn greet(path: web::Path<(String,)>) -> impl Responder {
    let name = &path.0;
    HttpResponse::Ok().json(format!("Hello, {}!", name))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").to(index))
            .service(web::resource("/greet/{name}").to(greet))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
