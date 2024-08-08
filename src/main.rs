use std::io::Error;
use std::fs;
use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use console::Term;
use dialoguer::Input;
use diesel::prelude::*;
use chrono::prelude::*;
use chrono;
use toml;


use DayList::Todo;
use DayList::Daylist;
use self::diesel_demo::*;
use self::models::*;

fn main() -> Result<(), Error> {
    let term = Term::stdout();
    let mut daylist = Daylist::new(); // !
    let mut todo_name: Vec<String> = Vec::new();
    let db_path = "daylist_db.toml";

    // read in file and put contents in daylist if db exists
    // create file and create a new daylist var if db does not exist

    // deserialize and load from toml
    let daylist_db = fs::read_to_string(db_path);
    match daylist_db {
        Ok(db) => daylist = toml::from_str(&db).expect("reads daylist from toml string"),
        Err(_) => {}
    }




    // running loop 
    loop {
        // x or q to exit 
        // a to add d to delete
        // e to edit 
        // c to complete a todo
        let nav_prompt = "\nNavigation:\na to add todo, d to delete todo\nc to complete a todo\ne to edit a todo\nx or q to exit program\n\n";

        print!("{}", daylist.show_daylist());

        let input: char = Input::new()
            .with_prompt(nav_prompt)
            .interact_text().expect("valid input type");

        match input {
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

            'x' => break,
            'q' => break,
            _ => println!("invalid input, try again"),
        }



    }


    // serialize and save to toml   - CREATE/UPDATE
    let daylist_toml = toml::to_string(&daylist).expect("converted daylist to toml");
    fs::write(&db_path, &daylist_toml).expect("saved daylist to db"); // unlikely to fail

    Ok(())
}



#[test]
fn test_db() {
    let connection = establish_connection();

    let new_todo = create_todo(
        &connection,
        "Learn Rust",
        false,
        Some("Focus on Diesel integration"),
        Local::today().naive_local(),
    );

    println!("Created new todo: {:?}", new_todo);
}




#[test]
fn print_todo() {
    let notes = Some(String::from("maybe minecraft"));
    let mut my_task = Todo::new(String::from("play games with Lyssi"), notes.clone()); 
    assert_eq!(my_task.notes, notes);
    my_task.mark_done();
    assert!(my_task.complete);
}

#[test]
fn mark_done() {
    let notes = Some(String::from(""));
    let mut my_task = Todo::new(String::from(""), notes.clone()); 
    my_task.mark_done();
    assert!(my_task.complete);
}

#[test]
fn add_remove() {
    let mut my_daylist = Daylist::new();
    let my_todo = Todo::new(String::from("go to gym"), Some(String::from("monday workout")) );
    my_daylist.add_todo(my_todo).expect("tried to add existing todo");
    my_daylist.remove_todo(String::from("go to gym")).expect("todo not found");
}

#[test]
fn show_daylist_test() {
    let mut my_daylist = Daylist::new();
    let todo_name = String::from("go to gym");
    let my_todo = Todo::new(todo_name, Some(String::from("monday workout")) );
    my_daylist.add_todo(my_todo).expect("tried to add existing todo");
    println!("{}", my_daylist.show_daylist());
}
