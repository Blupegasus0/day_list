use std::io::Error;
use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use console::Term;
use dialoguer::Input;

#[derive(Serialize, Deserialize, Clone)]
struct Todo {
    name: String,
    notes: Option<String>,
    complete: bool,
}

#[derive(Serialize, Deserialize, Clone)]
struct Daylist {
    date: String,
    todos: HashMap<String, Todo>,   
}

fn main() -> Result<(), Error> {
    let term = Term::stdout();
    let mut daylist = Daylist::new();
    
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


/*
    let todo_name = String::from("name");
    let date = &daylist.date;
    let name = &daylist.todos.get(&todo_name).unwrap().name;
    let notes = &daylist.todos.get(&todo_name).unwrap().notes;
*/

    print!("{}", daylist.show_daylist().unwrap());

    Ok(())
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

    fn show_daylist(&self) -> Option<String> {
        // print to term
        let mut daylist = String::new();

        for (name, todo) in self.todos.iter() {
            let note = format!("{}\n{}\n\nNotes: {}", 
                self.date, 
                name, 
                todo.notes.as_ref().unwrap()
            );

            daylist.push_str(&note);
            daylist.push_str("\n\n");
        }


        Some(daylist)
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

#[test]
fn show_daylist_test() {
    let mut my_daylist = Daylist::new();
    let todo_name = String::from("go to gym");
    let my_todo = Todo::new((todo_name), Some(String::from("monday workout")) );
    my_daylist.add_todo(my_todo).expect("tried to add existing todo");
    println!("{}", my_daylist.show_daylist().expect("todo not found"));
}
