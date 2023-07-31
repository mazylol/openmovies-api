// @generated automatically by Diesel CLI.

diesel::table! {
    boxoffice (boxoffice_id) {
        boxoffice_id -> Int4,
        movie_uuid -> Uuid,
        #[max_length = 50]
        budget -> Varchar,
        #[max_length = 50]
        us_gross -> Varchar,
        #[max_length = 50]
        worldwide_gross -> Varchar,
        #[max_length = 50]
        us_opening_weekend -> Varchar,
        #[max_length = 50]
        worldwide_opening_weekend -> Varchar,
    }
}

diesel::table! {
    cast (cast_id) {
        cast_id -> Int4,
        movie_uuid -> Uuid,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 100]
        role -> Varchar,
    }
}

diesel::table! {
    countryoforigin (country_id) {
        country_id -> Int4,
        movie_uuid -> Uuid,
        #[max_length = 2]
        country_code -> Varchar,
    }
}

diesel::table! {
    crew (crew_id) {
        crew_id -> Int4,
        movie_uuid -> Uuid,
        #[max_length = 50]
        crew_type -> Varchar,
        #[max_length = 100]
        crew_member_name -> Varchar,
    }
}

diesel::table! {
    distributor (distributor_id) {
        distributor_id -> Int4,
        movie_uuid -> Uuid,
        #[max_length = 100]
        distributor_name -> Varchar,
    }
}

diesel::table! {
    genre (genre_id) {
        genre_id -> Int4,
        movie_uuid -> Uuid,
        #[max_length = 50]
        genre_name -> Varchar,
    }
}

diesel::table! {
    movies (movie_uuid) {
        movie_uuid -> Uuid,
        #[max_length = 255]
        title -> Varchar,
        #[max_length = 100]
        shortname -> Varchar,
        description -> Text,
        #[max_length = 5]
        rating -> Varchar,
        #[max_length = 50]
        language -> Varchar,
    }
}

diesel::table! {
    release (release_id) {
        release_id -> Int4,
        movie_uuid -> Uuid,
        year -> Int4,
        month -> Int4,
        day -> Int4,
    }
}

diesel::table! {
    runtime (runtime_id) {
        runtime_id -> Int4,
        movie_uuid -> Uuid,
        hours -> Int4,
        minutes -> Int4,
        seconds -> Int4,
    }
}

diesel::joinable!(boxoffice -> movies (movie_uuid));
diesel::joinable!(cast -> movies (movie_uuid));
diesel::joinable!(countryoforigin -> movies (movie_uuid));
diesel::joinable!(crew -> movies (movie_uuid));
diesel::joinable!(distributor -> movies (movie_uuid));
diesel::joinable!(genre -> movies (movie_uuid));
diesel::joinable!(release -> movies (movie_uuid));
diesel::joinable!(runtime -> movies (movie_uuid));

diesel::allow_tables_to_appear_in_same_query!(
    boxoffice,
    cast,
    countryoforigin,
    crew,
    distributor,
    genre,
    movies,
    release,
    runtime,
);
