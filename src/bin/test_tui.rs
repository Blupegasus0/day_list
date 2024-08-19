use diesel::r2d2::{self, ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;
use std::env;

type DbPool = Pool<ConnectionManager<SqliteConnection>>;

fn establish_connection_pool(database_url: &str) -> DbPool {
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

fn main() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = establish_connection_pool(&database_url);

    // Example usage
    {
        let conn = pool.get().expect("Failed to get a connection from the pool.");
        // Perform your CRUD operation here
    }
}

