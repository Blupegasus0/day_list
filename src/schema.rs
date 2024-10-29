pub mod schema {
    #[derive(sqlx::FromRow)]
    pub struct Todo {
        todo_id: u32,
        title: String,
        description: String,
        date_created: NativeDateTime,
        status: bool,
        date_due: NataiveDateTime,
        reminder_date: NativeDateTime,
        parent_todo: u32,
        priority: i32,
        project_id: u32,
    }

    #[derive(sqlx::FromRow)]
    pub struct Project {
        project_id: u32,
        name: String,
        description: String,
        color: String,
        favorite_status: bool,
    }
}
