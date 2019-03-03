use std::env;
use crate::error::ShellError;

pub fn pwd(_args: &Vec<String>, _msg: Option<String>) -> Result<String, ShellError> {
    if let Ok(path) = env::current_dir() {
       return Ok(path.display().to_string());
    }
    Err(ShellError::Error("Unable to get current directory"))
}
