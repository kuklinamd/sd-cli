use crate::error::ShellError;
use crate::shell::ShellResult;

use std::env;
use std::path::PathBuf;

pub fn cd(args: &Vec<String>, _stdin: Option<String>) -> ShellResult<String> {
    if args.len() > 1 {
        return ShellResult::Err(ShellError::Error("Too many args"))
    } else if args.len() == 1 {
        let srcdir = PathBuf::from(&args[0]);
        if !env::set_current_dir(srcdir).is_ok() {
            return ShellResult::Err(ShellError::Error("Error"))
        }
    }
    ShellResult::Empty
}