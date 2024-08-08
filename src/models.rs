use diesel::prelude::*;
use chrono::NaiveDateTime;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::todo)]
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


