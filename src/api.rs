use actix_web::{web, get, post, delete, put, HttpResponse};

use crate::Database;

#[get("/movies")]
pub async fn get_movies(db: web::Data<Database>) -> HttpResponse {
    let movies = db.get_movies();
    HttpResponse::Ok().json(movies)
}