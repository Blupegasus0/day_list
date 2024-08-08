use chrono::NaiveDate;

#[derive(Queryable, Insertable, Debug)]
#[table_name = "todos"]
pub struct Todo {
    pub id: i32,
    pub name: String,
    pub completed: bool,
    pub notes: Option<String>,
    pub date: NaiveDate,
}
