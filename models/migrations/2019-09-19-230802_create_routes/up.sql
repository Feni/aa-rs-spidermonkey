CREATE TABLE routes (
    id SERIAL PRIMARY KEY, 
    app_id integer REFERENCES apps NOT NULL,
    name VARCHAR(64),
    raw VARCHAR(255) NOT NULL,
    pattern VARCHAR(255) NOT NULL,
    
    method_get boolean default TRUE NOT NULL,
    method_post boolean default TRUE NOT NULL,
    extra_methods smallint[],

    view_id integer REFERENCES views NOT NULL
)
