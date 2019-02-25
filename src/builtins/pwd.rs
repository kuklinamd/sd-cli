use std::env;

pub fn pwd(_args: &Vec<String>, _msg: Option<String>) -> Option<String> {
    if let Ok(path) = env::current_dir() {
       return Some(path.display().to_string());
    }
    None
}
