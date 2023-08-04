mod api;
mod models;
mod repository;

use api::user_api::{create_user};
use repository::mongodb_repo::MongoRepo;

use actix_web::{get, App, HttpServer, Responder};
use actix_web::web::Data;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(create_user) })
        .bind(("localhost", 8080))?
        .run()
        .await
}