pub mod models;
pub mod schema;
pub mod api;

use actix_web::{HttpServer, App, web, Responder, get, HttpResponse};
use ::api::Database;
use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

#[get("/health")]
async fn healthcheck() -> impl Responder {
    let response = Response {
        message: "Everything is working fine".to_string()
    };
    HttpResponse::Ok().json(response)
}

async fn not_found() -> impl Responder {
    let response = Response {
        message: "Resource not found".to_string(),
    };
    HttpResponse::NotFound().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::new();
    let app_data = web::Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(api::get_movies)
            .service(healthcheck)
            .default_service(web::route().to(not_found))
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}