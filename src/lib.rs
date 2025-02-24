pub mod schema;
pub mod db;
pub mod nav;
pub mod state;

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


pub mod utils {
    // TODO display popup message for user
    pub fn alert(message: &str) {}
}
