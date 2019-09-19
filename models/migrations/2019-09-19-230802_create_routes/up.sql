CREATE TABLE routes (
    id SERIAL PRIMARY KEY, 
    app_id integer REFERENCES apps,
    name VARCHAR(64),
    pattern VARCHAR(255) NOT NULL,
    view_id integer REFERENCES views
)

