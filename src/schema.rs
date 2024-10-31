pub mod schema {
    use chrono::NaiveDateTime;

    #[derive(sqlx::FromRow)]
    pub struct Todo {
        pub todo_id: i32,
        pub title: String,
        pub description: Option<String>,
        pub date_created: Option<NaiveDateTime>,
        pub status: i8, // errors i fear
        pub date_due: Option<NaiveDateTime>,
        pub reminder_date: Option<NaiveDateTime>,
        pub parent_todo: Option<i32>,
        pub priority: i32,
        pub project_id: Option<i32>,
    }

    #[derive(sqlx::FromRow)]
    pub struct Project {
        pub project_id: i32,
        pub name: String,
        pub description: Option<String>,
        pub color: Option<String>,
        pub favorite_status: bool,
    }
}
