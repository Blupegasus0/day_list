-- Add migration script here
CREATE TABLE todo (
  todo_id INT PRIMARY KEY AUTO_INCREMENT,
  title VARCHAR(100) NOT NULL,
  description TEXT,
  date_created DATE,
  status BOOLEAN NOT NULL,
  date_due DATE,
  reminder_date DATE,
  parent_todo INT,
  priority INT NOT NULL,
  project_id INT
);

CREATE TABLE project (
  project_id INT PRIMARY KEY AUTO_INCREMENT,
  name VARCHAR(100) NOT NULL,
  description TEXT,
  color VARCHAR(20),
  favorite_status BOOLEAN
);

ALTER TABLE todo ADD FOREIGN KEY (parent_todo) REFERENCES todo(todo_id);
ALTER TABLE todo ADD FOREIGN KEY (project_id) REFERENCES project(project_id);
