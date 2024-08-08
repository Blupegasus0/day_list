use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use chrono::prelude::*;
use crate::models::Todo;
use crate::schema::todos;

pub mod Daylist_db {
    pub fn establish_connection() -> PgConnection {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
    }    

}
