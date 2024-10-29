pub mod schema {
    use chrono::NaiveDateTime;

    #[derive(sqlx::FromRow)]
    pub struct Todo {
        pub todo_id: i32,
        pub title: String,
        pub description: Option<String>,
        pub date_created: NaiveDateTime,
        pub status: bool,
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


    impl Todo {
        pub new(
        title: String, description: Option<String>, date_created: NaiveDateTime,
        date_due: Option<NaiveDateTime>, reminder_date: Option<NaiveDateTime>,
        parent_todo: Option<i32>, priority: i32, project_id: Option<i32>
    ) -> self {
        Todo {
            id: 100, // not sure how to handle auto_increment
            title,
            description,
            date_created,
            status: false,
            date_due,
            reminder_date,
            parent_todo,
            priority,
            project_id,
        }
}
    }
}
