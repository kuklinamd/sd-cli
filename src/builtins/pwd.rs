use std::env;
use crate::error::ShellError;
use crate::shell::ShellResult;

pub fn pwd(_args: &Vec<String>, _msg: Option<String>) -> ShellResult<String> {
    if let Ok(path) = env::current_dir() {
       return ShellResult::Ok(path.display().to_string());
    }
    ShellResult::Err(ShellError::Error("Unable to get current directory"))
}
