// @generated automatically by Diesel CLI.

diesel::table! {
    movies (shortname) {
        #[max_length = 50]
        shortname -> Varchar,
        #[max_length = 255]
        title -> Varchar,
        description -> Text,
        #[max_length = 10]
        rating -> Varchar,
        #[max_length = 50]
        language -> Varchar,
        #[max_length = 10]
        box_office_budget -> Varchar,
        #[max_length = 10]
        box_office_gross_us -> Varchar,
        #[max_length = 10]
        box_office_gross_worldwide -> Varchar,
        #[max_length = 10]
        box_office_opening_weekend_us -> Varchar,
        #[max_length = 10]
        box_office_opening_weekend_worldwide -> Varchar,
        release_year -> Int4,
        release_month -> Int4,
        release_day -> Int4,
        runtime_hours -> Int4,
        runtime_minutes -> Int4,
        runtime_seconds -> Int4,
        director -> Array<Nullable<Text>>,
        producer -> Array<Nullable<Text>>,
        writer -> Array<Nullable<Text>>,
        cast -> Array<Nullable<Text>>,
        genre -> Array<Nullable<Text>>,
        distributor -> Array<Nullable<Text>>,
        country_of_origin -> Array<Nullable<Text>>,
    }
}
