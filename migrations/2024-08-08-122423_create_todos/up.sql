-- Your SQL goes here
CREATE TABLE todos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    completed BOOLEAN NOT NULL,
    notes TEXT,
    date DATE NOT NULL
);

