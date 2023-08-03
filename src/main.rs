mod api;
mod models;
mod repository;

use api::user_api::{create_user};
use repository::mongodb_repo::MongoRepo;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json("Hello from Rust and MongoDB")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind(("localhost", 8080))?
        .run()
        .await
}