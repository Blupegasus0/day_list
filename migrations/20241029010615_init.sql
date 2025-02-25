-- Add migration script here
CREATE TABLE todo (
  todo_id INTEGER PRIMARY KEY AUTOINCREMENT,
  title TEXT NOT NULL,
  description TEXT,
  date_created INTEGER,
  status BOOLEAN NOT NULL,
  date_due INTEGER,
  reminder_date INTEGER,
  parent_todo INTEGER,
  priority INTEGER NOT NULL,
  project_id INTEGER,
  FOREIGN KEY (parent_todo) REFERENCES todo(todo_id),
  FOREIGN KEY (project_id) REFERENCES project(project_id)
);

CREATE TABLE project (
  project_id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  description TEXT,
  color TEXT,
  favorite_status BOOLEAN
);
