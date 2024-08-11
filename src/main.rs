use std::io::Error;

use console::Term;
use dialoguer::Input;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use chrono;

//mod schema;
//mod models;

fn main() -> Result<(), Error> {
    let term = Term::stdout();
    let mut todo_name: Vec<String> = Vec::new();

    // read in file and put contents in daylist if db exists
    // create file and create a new daylist var if db does not exist


    // running loop 
    loop {
        // x or q to exit 
        // a to add d to delete
        // e to edit 
        // c to complete a todo
        let nav_prompt = "\nNavigation:\na to add todo, d to delete todo\nc to complete a todo\ne to edit a todo\nx or q to exit program\n\n";

        // show day_list

        let input: char = Input::new()
            .with_prompt(nav_prompt)
            .interact_text().expect("valid input type");

        match input {
            /*
            'a' => {
                let _ = term.write_line("To Create a Todo, enter a name");
                let name: String = Input::new()
                    .with_prompt("Todo Name?")
                    .interact_text()
                    .unwrap();
                let notes: String = Input::new()
                    .with_prompt("Any notes?")
                    .interact_text()
                    .unwrap();

                let todo = Todo::new(name, Some(notes));
                let _ = daylist.add_todo(todo)?;
            },

            'd' => {
                let name_to_remove: String = Input::new()
                    .with_prompt("Todo Name to remove?")
                    .interact_text()
                    .unwrap();

                loop {
                    println!("Remove {}?", name_to_remove);
                    let confirm: char = Input::new()
                        .interact_text()
                        .unwrap();

                    match confirm {
                        'y' => {
                            let _ = daylist.remove_todo(name_to_remove.to_lowercase())?;
                            println!("removed {}.", name_to_remove);
                            break;
                        },
                        'n' => {
                            let _ = term.write_line("nothing removed");
                            break;
                        },
                        _ => {
                            let _ = term.write_line("Invalid response, try again");
                        }

                    }

                }
            },

            'c' => {
                // find a way to display the completion status of todos
                let name_to_complete: String = Input::new()
                    .with_prompt("Todo Name to complete?")
                    .interact_text()
                    .unwrap();

                let _ = daylist.complete_todo(name_to_complete.to_lowercase())?;
            },

            'e' => {
                //delete and add back
                let name_to_remove: String = Input::new()
                    .with_prompt("Todo Name to edit?")
                    .interact_text()
                    .unwrap();

                loop {
                    let _ = daylist.remove_todo(name_to_remove.to_lowercase())?;

                    println!("Edit {}?", name_to_remove);
                    let confirm: char = Input::new()
                        .interact_text()
                        .unwrap();

                    match confirm {
                        'y' => {
                            let name: String = Input::new()
                                .with_prompt("Todo Name?")
                                .interact_text()
                                .unwrap();
                            let notes: String = Input::new()
                                .with_prompt("Any notes?")
                                .interact_text()
                                .unwrap();

                            let todo = Todo::new(name, Some(notes));
                            let _ = daylist.add_todo(todo.clone())?;

                            println!("updated {} to {}.", name_to_remove, todo.get_name());

                            break;
                        },
                        'n' => {
                            let _ = term.write_line("nothing changed");
                            break;
                        },
                        _ => {
                            let _ = term.write_line("Invalid response, try again");
                        }

                    }
                }
            },
        */

            'x' => break,
            'q' => break,
            _ => println!("invalid input, try again"),
        }



    }

    println!("Running Tests...");
    db_read();


    Ok(())
}



fn db_read() {
// -- Read
    use DayList::schema;
    use DayList::establish_connection;
    use DayList::models::NewTodo;
    use DayList::models::Todo;

    let connection = &mut establish_connection();
    let results = schema::todo::table
        .select(Todo::as_select())
        .load(connection)
        .expect("Error loading todos");

    println!("Displaying {} todos", results.len());
    for t in results {
        println!("{}", t.title);
        println!("-----------\n");
        println!("{}", t.description.unwrap());
    }
    

}

fn db_create() {
    // -- Create
    use DayList::schema;
    use DayList::establish_connection;
    use DayList::models::NewTodo;
    use DayList::models::Todo;

    let mut title = String::from("New todo");
    let mut description = String::from("I am testing the db");
    let mut completed = false;

    let new_todo = NewTodo { 
        title: title, 
        description: Some(description), 
        completed: false, 
        parent_todo_id: None 
    };
    let connection = &mut establish_connection();

    diesel::insert_into(schema::todo::table)
        .values(&new_todo)
        .execute(connection)
        .expect("Error saving new todo");
}

fn db_update() {
// -- Update
    use DayList::schema;
    use DayList::establish_connection;
    use DayList::models::NewTodo;
    use DayList::models::Todo;

    let connection = &mut establish_connection();
    let id = 1;

    let todo = diesel::update(schema::todo::table.find(id))
        .set(schema::todo::completed.eq(true))
        .execute(connection)
        .unwrap();
    // println!("Completed '{}'", todo.title);


}

fn db_delete() {
    // -- Delete
    use DayList::schema;
    use DayList::establish_connection;
    use DayList::models::NewTodo;
    use DayList::models::Todo;

    let connection = &mut establish_connection();
    let target = String::from("test");
    let pattern = format!("%{}%", target);

    let num_deleted = diesel::delete(schema::todo::table.filter(schema::todo::title.like(pattern)))
        .execute(connection)
        .expect("Error deleting posts");

    println!("Deleted {} todos", num_deleted);

}

