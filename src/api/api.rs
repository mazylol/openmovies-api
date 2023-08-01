use actix_web::{get, post, web, HttpResponse};

use crate::{models::movie::Movie, repository::database::Database};

#[post("/movies")]
pub async fn create_movie(db: web::Data<Database>, new_movie: web::Json<Movie>) -> HttpResponse {
    let movie = db.create_movie(new_movie.into_inner());
    match movie {
        Ok(movie) => HttpResponse::Ok().json(movie),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/movies")]
pub async fn get_movies(db: web::Data<Database>) -> HttpResponse {
    let movies = db.get_movies();
    HttpResponse::Ok().json(movies)
}

#[get("/movies/{id}")]
pub async fn get_movie_by_shortname(
    db: web::Data<Database>,
    id: web::Path<String>
) -> HttpResponse {
    let movie = db.get_movie_by_shortname(&id);

    match movie {
        Some(movie) => HttpResponse::Ok().json(movie),
        None => HttpResponse::NotFound().body("Movie not found"),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").service(get_movies).service(create_movie).service(get_movie_by_shortname));
}