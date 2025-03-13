use crate::model::schema::Todo;
use chrono::NaiveDateTime;

pub enum EditSelection {
    Name,
    Description,
    DateDue,
    ReminderDate,
    Priority,
}

pub enum EditAction {
    Create,
    Update,
}

pub struct EditTodo {
    pub selection: EditSelection,
    pub name: String,
    pub description: String,
    pub date_due: String,
    pub reminder_date: String,
    pub priority: i64,
}

impl Default for EditTodo {
    fn default() -> Self {
        Self::new()
    }
}

impl EditTodo {
    pub fn new() -> EditTodo {
        EditTodo {
            selection: EditSelection::Name,
            name: String::new(),
            description: String::new(),
            date_due: String::new(),
            reminder_date: String::new(),
            priority: 4,
        }
    }

    pub fn update_todo(&mut self) -> Todo {
        // TODO bring in edit_string
        // TODO set edit string to current todo data

        let update_todo = Todo {
            todo_id: 0,
            title: self.name.clone(), 
            description: Some(self.description.clone()),
            date_created: None,
            status: 0,
            date_due: self.parse_due(), 
            reminder_date: self.parse_reminder(), 
            parent_todo: None, 
            priority: self.priority, 
            project_id: None,
        };

        self.reset();

        update_todo
    }

    pub fn new_todo(&mut self) -> Todo {
        let new_todo = Todo {
            todo_id: 0,
            title: self.name.clone(), 
            description: Some(self.description.clone()),
            date_created: None,
            status: 0,
            date_due: self.parse_due(), 
            reminder_date: self.parse_reminder(), 
            parent_todo: None, 
            priority: self.priority, 
            project_id: None,
        };

        self.reset();

        new_todo
    }

    fn reset(&mut self) {
        self.name.clear();
        self.description.clear();
        self.date_due.clear();
        self.reminder_date.clear();
        self.priority = 4; // Magic Number
        self.selection = EditSelection::Name;
    }

    fn parse_due(&self) -> Option<NaiveDateTime> {
        NaiveDateTime::parse_from_str(self.date_due.as_str(), "%d/%m/%y %H:%M").ok()
    }
    fn parse_reminder(&self) -> Option<NaiveDateTime> {
        NaiveDateTime::parse_from_str(self.reminder_date.as_str(), "%d/%m/%y %H:%M").ok()
    }

}


