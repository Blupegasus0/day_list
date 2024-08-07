use std::io::Error;
use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Todo {
    name: String,
    notes: Option<String>,
    complete: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Daylist {
    date: String,
    todos: HashMap<String, Todo>,   
}


impl Todo {
    // CREATE
    pub fn new(name: String, notes: Option<String>) -> Todo {
        Todo {
            name,
            notes,
            complete: false,
        }
    }

    // UPDATE
    pub fn edit_todo(&self, ) -> Result<(), Error> {
        todo!();
    }

    // UPDATE
    pub fn mark_done(&mut self) {
        self.complete = true;
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_notes(&self) -> String {
        match self.notes.clone() {
            Some(note) => note,
            _ => String::from(""),
        }
    }

}

impl Daylist {
    // CREATE
    pub fn new() -> Daylist {
        Daylist {
            date: String::from("today"),
            todos: HashMap::new(),
        }

    }

    // DELETE
    pub fn remove_todo(&mut self, name: String) -> Result<String, Error> {
        match self.todos.remove(&name) {
            Some(k) => Ok(format!("Removed todo: {}",k.name)),
            None => Err(Error::other("todo not found")),
        }
    }

    // CREATE
    pub fn add_todo(&mut self, todo: Todo) -> Result<(), Error> {
        if self.todos.contains_key(&todo.name) {
            Err(Error::other("todo already exists, remove to update"))
        } else {
            self.todos.insert(todo.name.to_lowercase().clone(), todo);
            Ok(())
        }
    }

    // UPDATE
    pub fn complete_todo(&mut self, name: String) -> Result<(), Error> {
        match self.todos.get_mut(&name) {
            Some(todo) => Ok(todo.mark_done()),
            None => Err(Error::other("todo not found")),
        }

    }

    // CREATE
    pub fn save_daylist() -> Result<(), Error> {
        // write to toml file
        todo!();
    }

    // READ
    pub fn load_daylist() -> Result<(), Error> {
        // load from toml file
        todo!();
    }

    pub fn show_daylist(&self) -> String {
        // print to term
        let mut daylist = String::new();

        for (name, todo) in self.todos.iter() {
            let mut completion = "[ ]";
            if todo.complete == true {
                completion = "[x]";
            }

            let note = format!("\n{}\n{} {}\nNotes: {}", 
                self.date, 
                completion,
                todo.name, 
                todo.notes.as_ref().unwrap()
            );

            daylist.push_str(&note);
            daylist.push_str("\n\n");
        }

        daylist
    }

}

