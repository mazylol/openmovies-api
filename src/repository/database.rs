use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use std::fmt::Error;

use crate::models::movie::Movie;
use crate::repository::schema::movies::dsl::*;

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct Database {
    pool: DBPool,
}

impl Database {
    pub fn new() -> Self {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool: DBPool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        Database { pool }
    }

    pub fn get_movies(&self) -> Vec<Movie> {
        movies
            .load::<Movie>(&mut self.pool.get().unwrap())
            .expect("Error loading all todos")
    }

    pub fn create_movie(&self, movie: Movie) -> Result<Movie, Error> {
        let movie = Movie { ..movie };
        diesel::insert_into(movies)
            .values(&movie)
            .execute(&mut self.pool.get().unwrap())
            .expect("Error creating new movie");
        Ok(movie)
    }

    pub fn get_movie_by_shortname(&self, id: &str) -> Option<Movie> {
        let movie = movies
            .find(id)
            .get_result::<Movie>(&mut self.pool.get().unwrap())
            .expect("Error loading movie");

        Some(movie)
    }
}
