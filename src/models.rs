use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::schema;

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::todo)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    //pub date_created: NaiveDateTime,
    pub completed: bool,
    //pub due_date: NaiveDateTime,
    //pub reminder_date: NaiveDateTime,
    pub parent_todo_id: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = schema::todo)]
pub struct NewTodo {
    pub title: String,
    pub description: Option<String>,
    //pub date_created: NaiveDateTime,
    pub completed: bool,
    //pub due_date: NaiveDateTime,
    //pub reminder_date: NaiveDateTime,
    pub parent_todo_id: Option<i32>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::label)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Label {
    pub id: i32,
    pub name: String,
    pub color: Option<String>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::todo_label)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct todo_label {
    pub todo_id: Option<i32>,
    pub label_id: Option<i32>,
}
