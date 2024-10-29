use sqlx::mysql::MySqlRow;
use sqlx::Row;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool  = MySqlPool::connect(&database_url).await?;
    
    let rows: Vec<MySqlRow> = sqlx::query("SELECT * FROM todo")
                                .fetch_all($pool)
                                .await?;

    for row in rows {
        let id: i32 = row.try_get("id")?;
        let name: &str = row.try_get("name")?;
        
        println!("ID: {}, Name: {}", id, name);
    }

    Ok(())
}
