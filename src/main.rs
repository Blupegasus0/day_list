use std::io::Error;
use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use console::Term;
use dialoguer::Input;

fn main() {
    let mut daylist: Vec<Todo>;
}

#[derive(Serialize, Deserialize, Clone)]
struct Todo {
    name: String,
    notes: Option<String>,
    complete: bool,
}

#[derive(Serialize, Deserialize)]
struct Daylist {
    date: String,
    // todos: Option<HashMap<String, Todo>>,   
    todos: HashMap<String, Todo>,   
}


impl Todo {
    fn new(name: String, notes: Option<String>) -> Todo {
        Todo {
            name,
            notes,
            complete: false,
        }
    }

    fn edit_todo(&self, ) -> Result<(), Error> {
        todo!();
    }

    fn mark_done(&mut self) {
        self.complete = true;
    }

}

impl Daylist {
    fn new() -> Daylist {
        Daylist {
            date: String::from("today"),
            todos: HashMap::new(),
        }
        
    }

    fn remove_todo(&mut self, name: String) -> Result<String, Error> {
        match self.todos.remove(&name) {
            Some(k) => Ok(format!("Removed todo: {}",k.name)),
            None => Err(Error::other("todo not found")),
        }
    }

    fn add_todo(&mut self, todo: Todo) -> Result<(), Error> {
        if self.todos.contains_key(&todo.name) {
            Err(Error::other("todo already exists, remove to update"))
        } else {
            self.todos.insert(todo.name.clone(), todo);
            Ok(())
        }
    }

    fn save_daylist() -> Result<(), Error> {
        // write to toml file
        todo!();
    }

    fn load_daylist() -> Result<(), Error> {
        // load from toml file
        todo!();
    }

    fn show_daylist() -> Result<(), Error> {
        // print to term
        Ok(())
    }

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
