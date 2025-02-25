-- Add migration script here
DROP TABLE todo;

CREATE TABLE todo (
  todo_id INTEGER PRIMARY KEY AUTOINCREMENT,
  title TEXT NOT NULL,
  description TEXT,
  date_created TEXT,
  status BOOLEAN NOT NULL,
  date_due TEXT,
  reminder_date TEXT,
  parent_todo INTEGER,
  priority INTEGER NOT NULL,
  project_id INTEGER,
  FOREIGN KEY (parent_todo) REFERENCES todo(todo_id),
  FOREIGN KEY (project_id) REFERENCES project(project_id)
);
