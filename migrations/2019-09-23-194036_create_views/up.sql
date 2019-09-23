CREATE TABLE views (
    id SERIAL PRIMARY KEY, 
    app_id integer REFERENCES apps NOT NULL,
    name VARCHAR(64) NOT NULL,
    kind smallint NOT NULL,
    content_url VARCHAR(255),
    content TEXT,

    created_at timestamp default current_timestamp NOT NULL,
    updated_at timestamp default current_timestamp NOT NULL
)
