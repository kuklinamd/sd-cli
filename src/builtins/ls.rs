use crate::error::ShellError;
use crate::shell::ShellResult;

use std::env;
use std::path::PathBuf;
use std::fs;
use std::path::Path;

/// Print folder content
pub fn ls(args: &Vec<String>, _stdin: Option<String>) -> ShellResult<String> {
    if args.len() > 1 {
        return ShellResult::Err(ShellError::Error("ls: too many arguments"));
    } else if args.len() == 1 {
        let dir = PathBuf::from(&args[0]);
        return ShellResult::Ok(visit_dirs(&dir).join("\n"));
    } else {
        match env::current_dir() {
            Ok(dir) => ShellResult::Ok(visit_dirs(&dir).join("\n")),
            Err(_err) => ShellResult::Err(ShellError::Error("ls: could not get current dirrectory")),
        }
    }
}

/// Function returns all filenames in given path
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

#[cfg(test)]
mod test_ls {
    use super::*;

    #[test]
    fn test_content() {
        match ls(&vec![".".to_string()], Option::None) {
            ShellResult::Ok(res) => assert!(res.len() > 0),
            ShellResult::Err(_) => assert!(false)
        };

        match ls(&vec![], Option::None) {
            ShellResult::Ok(res) => assert!(res.len() > 0),
            ShellResult::Err(_) => assert!(false)
        };

        match ls(&vec!["/%@$#lstestfoldernotexist@#@%)_/".to_string()], Option::None) {
            ShellResult::Ok(res) => assert_eq!(0, res.len()),
            ShellResult::Err(_) => assert!(false)
        };
    }    
}
