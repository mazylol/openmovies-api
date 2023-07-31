pub mod models;
pub mod schema;
pub mod api;

use diesel::RunQueryDsl;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenvy::dotenv;
use models::Movies;
use std::env;

use crate::schema::movies::dsl::movies;

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

    pub fn get_movies(&self) -> Vec<Movies> {
        movies
            .load::<Movies>(&mut self.pool.get().unwrap())
            .expect("Error loading all todos")
    }
}