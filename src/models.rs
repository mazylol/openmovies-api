use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::boxoffice)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BoxOffice {
    pub boxoffice_id: i32,
    pub budget: String,
    pub us_gross: String,
    pub worldwide_gross: String,
    pub us_opening_weekend: String,
    pub worldwide_opening_weekend: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::cast)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Cast {
    pub cast_id: i32,
    pub movie_uuid: Uuid,
    pub name: String,
    pub role: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::countryoforigin)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CountryOfOrigin {
    pub country_id: i32,
    pub movie_uuid: Uuid,
    pub country_code: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::crew)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Crew {
    pub crew_id: i32,
    pub movie_uuid: Uuid,
    pub crew_type: String,
    pub crew_member_name: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::distributor)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Distributor {
    pub distributor_id: i32,
    pub movie_uuid: Uuid,
    pub distributor_name: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::genre)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Genre {
    pub genre_id: i32,
    pub movie_uuid: Uuid,
    pub genre_name: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::movies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Movies {
    pub movie_uuid: Uuid,
    pub title: String,
    pub shortname: String,
    pub description: String,
    pub rating: String,
    pub language: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::release)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Release {
    pub release_id: i32,
    pub year: i32,
    pub month: i32,
    pub day: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::runtime)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Runtime {
    pub runtime_id: i32,
    pub hours: i32,
    pub minutes: i32,
    pub seconds: i32,
}