DROP TABLE IF EXISTS users;
CREATE TABLE IF NOT EXISTS users 
(
    id BIGSERIAL PRIMARY KEY,
    username VARCHAR(32) UNIQUE,
    elo integer,
    wins integer,
    losses integer,
    draws integer
);

DROP TABLE IF EXISTS auth_infos;
CREATE TABLE IF NOT EXISTS auth_infos
(
	user_id bigint NOT NULL,
    password_hash VARCHAR(255),
	refresh_token VARCHAR(255),
	CONSTRAINT fk_user_id FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);