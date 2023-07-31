-- Table: Movies
CREATE TABLE movies (
    movie_uuid UUID PRIMARY KEY NOT NULL,
    title VARCHAR(255) NOT NULL,
    shortname VARCHAR(100) NOT NULL,
    description TEXT NOT NULL,
    rating VARCHAR(5) NOT NULL,
    language VARCHAR(50) NOT NULL
);

-- Table: BoxOffice
CREATE TABLE boxoffice (
    boxoffice_id SERIAL PRIMARY KEY NOT NULL,
    movie_uuid UUID NOT NULL,
    budget VARCHAR(50) NOT NULL,
    us_gross VARCHAR(50) NOT NULL,
    worldwide_gross VARCHAR(50) NOT NULL,
    us_opening_weekend VARCHAR(50) NOT NULL,
    worldwide_opening_weekend VARCHAR(50) NOT NULL,
    FOREIGN KEY (movie_uuid) REFERENCES movies(movie_uuid)
);

-- Table: Release
CREATE TABLE release (
    release_id SERIAL PRIMARY KEY NOT NULL,
    movie_uuid UUID NOT NULL,
    year INT NOT NULL,
    month INT NOT NULL,
    day INT NOT NULL,
    FOREIGN KEY (movie_uuid) REFERENCES movies(movie_uuid)
);

-- Table: Runtime
CREATE TABLE runtime (
    runtime_id SERIAL PRIMARY KEY NOT NULL,
    movie_uuid UUID NOT NULL,
    hours INT NOT NULL,
    minutes INT NOT NULL,
    seconds INT NOT NULL,
    FOREIGN KEY (movie_uuid) REFERENCES movies(movie_uuid)
);

-- Table: Genre
CREATE TABLE genre (
    genre_id SERIAL PRIMARY KEY NOT NULL,
    movie_uuid UUID NOT NULL,
    genre_name VARCHAR(50) NOT NULL,
    FOREIGN KEY (movie_uuid) REFERENCES movies(movie_uuid)
);

-- Table: "cast"
CREATE TABLE "cast" (
    cast_id SERIAL PRIMARY KEY NOT NULL,
    movie_uuid UUID NOT NULL,
    name VARCHAR(100) NOT NULL,
    role VARCHAR(100) NOT NULL,
    FOREIGN KEY (movie_uuid) REFERENCES movies(movie_uuid)
);

-- Table: Crew
CREATE TABLE crew (
    crew_id SERIAL PRIMARY KEY NOT NULL,
    movie_uuid UUID NOT NULL,
    crew_type VARCHAR(50) NOT NULL,
    crew_member_name VARCHAR(100) NOT NULL,
    FOREIGN KEY (movie_uuid) REFERENCES movies(movie_uuid)
);

-- Table: Distributor
CREATE TABLE distributor (
    distributor_id SERIAL PRIMARY KEY NOT NULL,
    movie_uuid UUID NOT NULL,
    distributor_name VARCHAR(100) NOT NULL,
    FOREIGN KEY (movie_uuid) REFERENCES movies(movie_uuid)
);

-- Table: CountryOfOrigin
CREATE TABLE countryoforigin (
    country_id SERIAL PRIMARY KEY NOT NULL,
    movie_uuid UUID NOT NULL,
    country_code VARCHAR(2) NOT NULL,
    FOREIGN KEY (movie_uuid) REFERENCES movies(movie_uuid)
);
