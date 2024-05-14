use std::io::Error;

use serde::{Serialize, Deserialize};
use console::Term;
use dialoguer::Input;

fn main() {
    let mut daylist: Vec<Todo>;
}

#[derive(Serialize, Deserialize)]
struct Todo {
    id: usize,
    name: String,
    notes: Option<String>,
    complete: bool,
}


impl Todo {
    fn new(id: usize, name: String, notes: Option<String>) -> Todo {
        Todo {
            id,
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

fn remove_todo(day_list: Vec<Todo>) -> Result<(), Error> {
    todo!();
}

fn save_daylist() -> Result<(), Error> {
    todo!();
}

fn load_daylist() -> Result<(), Error> {
    todo!();
}

fn show_daylist() -> Result<(), Error> {
    Ok(())
}

#[test]
fn print_todo() {
    let notes = Some(String::from("maybe minecraft"));
    let mut my_task = Todo::new(1, String::from("play games with Lyssi"), notes.clone()); 
    assert_eq!(my_task.notes, notes);
    my_task.mark_done();
    assert!(my_task.complete);
}

#[test]
fn mark_done() {
    let notes = Some(String::from(""));
    let mut my_task = Todo::new(1, String::from(""), notes.clone()); 
    my_task.mark_done();
    assert!(my_task.complete);
}
