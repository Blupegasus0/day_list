use rusqlite::{Connection, Result};
use std::io::{self, Write};

/// Function to fetch a batch of items from the database.
/// Limit the number of items fetched to enable lazy loading.
fn fetch_items(conn: &Connection, offset: i64, limit: i64) -> Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT item FROM items LIMIT ? OFFSET ?")?;
    let rows = stmt.query_map([limit, offset], |row| {
        let item: String = row.get(0)?;
        Ok(item)
    })?;
    
    let mut items = Vec::new();
    for item in rows {
        items.push(item?);
    }
    Ok(items)
}

/// Function to write items to the terminal.
fn write_to_terminal(items: &[String]) {
    for item in items {
        println!("{}", item);
    }
}

fn main() -> Result<()> {
    // Open a connection to the database.
    let conn = Connection::open("example.db")?;

    let limit = 10;
    let mut offset = 0;

    loop {
        // Fetch a batch of items from the database.
        let items = fetch_items(&conn, offset, limit)?;

        // If no more items are returned, break the loop.
        if items.is_empty() {
            println!("No more items to display.");
            break;
        }

        // Write the fetched items to the terminal.
        write_to_terminal(&items);

        // Wait for the user to press Enter to load more items.
        println!("Press Enter to load more items, or type 'exit' to quit:");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if input.trim().eq_ignore_ascii_case("exit") {
            break;
        }

        // Increment the offset to fetch the next batch of items.
        offset += limit;
    }

    Ok(())
}

