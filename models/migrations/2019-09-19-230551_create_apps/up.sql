-- Your SQL goes here
CREATE TABLE apps (
    id SERIAL PRIMARY KEY, 
    name VARCHAR(64) NOT NULL,
    domain VARCHAR NOT NULL,
    environment smallint default 0 NOT NULL,

    created_at timestamp default current_timestamp NOT NULL,
    updated_at timestamp default current_timestamp NOT NULL
)
