use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = crate::repository::schema::movies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Movie {
    pub shortname: String,
    pub title: String,
    pub description: String,
    pub rating: String,
    pub language: String,
    pub box_office_budget: String,
    pub box_office_gross_us: String,
    pub box_office_gross_worldwide: String,
    pub box_office_opening_weekend_us: String,
    pub box_office_opening_weekend_worldwide: String,
    pub release_year: i32,
    pub release_month: i32,
    pub release_day: i32,
    pub runtime_hours: i32,
    pub runtime_minutes: i32,
    pub runtime_seconds: i32,
    pub director: Vec<Option<String>>,
    pub producer: Vec<Option<String>>,
    pub writer: Vec<Option<String>>,
    pub cast: Vec<Option<String>>,
    pub genre: Vec<Option<String>>,
    pub distributor: Vec<Option<String>>,
    pub country_of_origin: Vec<Option<String>>,
}
