-- Your SQL goes here
CREATE TABLE todos (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    completed BOOLEAN NOT NULL,
    notes TEXT,
    date DATE NOT NULL
);
