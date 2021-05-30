CREATE TABLE IF NOT EXISTS users 
(
    id BIGSERIAL PRIMARY KEY,
    username VARCHAR(32) UNIQUE,
    elo integer,
    wins integer,
    losses integer,
    draws integer,
    password_hash VARCHAR(255),
    salt VARCHAR(255)
);