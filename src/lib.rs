pub mod model;
pub mod view;
pub mod controller;


//https://patorjk.com/software/taag/#p=display&f=Tmplr&t=Daylist
pub const LOGO: &str = "
  ____          _ _     _   
 |    \\ ___ _ _| |_|___| |_ 
 |  |  | .'| | | | |_ -|  _|
 |____/|__,|_  |_|_|___|_|  
           |___|            

";
pub const LOGO2: &str = "
   ____              _ _     _   
  |  _ \\  __ _ _   _| (_)___| |_ 
  | | | |/ _` | | | | | / __| __|
  | |_| | (_| | |_| | | \\__ \\ |_ 
  |____/ \\__,_|\\__, |_|_|___/\\__|
               |___/             
";
pub const LOGO3: &str = "
┳┓    ┓•  
┃┃┏┓┓┏┃┓┏╋
┻┛┗┻┗┫┗┗┛┗
     ┛    
";
pub const LOGO4: &str = "
  ___            _ _    _   
 |   \\ __ _ _  _| (_)__| |_ 
 | |) / _` | || | | (_-<  _|
 |___/\\__,_|\\_, |_|_/__/\\__|
            |__/            
";


impl crate::model::schema::Todo {
    pub fn format(&self, /* options */) -> String {
        let mut todo_status = "[ ]";
        let description = match self.description.clone() {
            Some(s) => s,
            None => "--".to_string(),
        }; 
        let date_due = match self.date_due {
            Some(d) => d.format("%d/%m/%Y %H:%M:%S").to_string(),
            _ => String::from("invalid date")
        };
        let reminder_date = match self.reminder_date {
            Some(d) => d.format("%d/%m/%Y %H:%M:%S").to_string(),
            _ => String::from("invalid date")
        };


        if self.status == 1 {
            todo_status = "[]";
        }

        format!("\n   {} {}\n       {}\n    {}\n    {}\n    {}\n",
            todo_status, self.title, description,
            reminder_date, date_due, self.priority
        )
    }
}

pub mod utils {
    // TODO display popup message for user
    pub fn alert(message: &str) {}
}
