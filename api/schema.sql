DROP TABLE IF EXISTS users, achievements;

CREATE TABLE users (
    id serial PRIMARY KEY,
    username VARCHAR(100) UNIQUE,
    password_hash VARCHAR(200),
    achievements integer[], 
    timestamps bigint[]
);

CREATE TABLE achievements (
    id serial PRIMARY KEY,
    name VARCHAR(100),
    description VARCHAR(500),
    image text
);