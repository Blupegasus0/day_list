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


    impl Todo {
        pub fn format(&self, /* options */) -> String {
            let mut todo_status = "[ ]";
            let description = match self.description.clone() {
                Some(s) => s,
                None => "--".to_string(),
            }; 
            let date_due = match self.date_due {
                Some(d) => d.format("%d/%m/%Y %H:%M:%S").to_string(),
                _ => String::from("invalid date")
            };
            let reminder_date = match self.reminder_date {
                Some(d) => d.format("%d/%m/%Y %H:%M:%S").to_string(),
                _ => String::from("invalid date")
            };


            if self.status == 1 {
                todo_status = "[]";
            }

            format!("\n   {} {}\n       {}\n    {}\n    {}\n    {}\n",
                todo_status, self.title, description,
                reminder_date, date_due, self.priority
            )
        }
    }
}
