use crate::error::ShellError;
use crate::shell::ShellResult;

use std::env;
use std::path::PathBuf;
use std::fs;
use std::path::Path;

pub fn ls(args: &Vec<String>, _stdin: Option<String>) -> ShellResult<String> {
    if args.len() > 1 {
        return ShellResult::Err(ShellError::Error("Too many args"));
    } else if args.len() == 1 {
        let dir = PathBuf::from(&args[0]);
        return ShellResult::Ok(visit_dirs(&dir).join("\n"));
    } else {
        match env::current_dir() {
            Ok(dir) => ShellResult::Ok(visit_dirs(&dir).join("\n")),
            Err(_err) => ShellResult::Err(ShellError::Error("Error")),
        }
    }
}

fn visit_dirs(dir: &Path) -> Vec<String> {
    let mut result = vec![];
    if dir.is_dir() {
        if let Ok(iterator) = fs::read_dir(dir) {
            for entry in iterator {
                if let Ok(entry) = entry {
                    result.push(String::from(entry.path().to_str().unwrap()));
                }
            }
        }
    }
    return result;
}

