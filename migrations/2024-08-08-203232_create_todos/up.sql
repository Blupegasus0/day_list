-- Your SQL goes here
CREATE TABLE todo (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    --date_created INTEGER DEFAULT (datetime('now')),
    completed BOOLEAN NOT NULL CHECK (completed IN (0, 1)),
    --due_date INTEGER,
    --reminder_date INTEGER,
    parent_todo_id INTEGER,
    FOREIGN KEY (parent_todo_id) REFERENCES todo(id) ON DELETE CASCADE
);

CREATE TABLE label (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL UNIQUE,
    color TEXT
);

CREATE TABLE todo_label (
    todo_id INTEGER,
    label_id INTEGER,
    PRIMARY KEY (todo_id, label_id),
    FOREIGN KEY (todo_id) REFERENCES todo(id) ON DELETE CASCADE,
    FOREIGN KEY (label_id) REFERENCES label(id) ON DELETE CASCADE
);
