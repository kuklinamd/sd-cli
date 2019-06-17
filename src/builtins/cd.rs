use crate::error::ShellError;
use crate::shell::ShellResult;

use std::env;
use std::path::PathBuf;

/// Change working directory to specfied
pub fn cd(args: &Vec<String>, _stdin: Option<String>) -> ShellResult<String> {
    if args.len() > 1 {
        return ShellResult::Err(ShellError::Error("cd: too many arguments"))
    } else if args.len() == 1 {
        let srcdir = PathBuf::from(&args[0]);
        if !env::set_current_dir(srcdir).is_ok() {
            return ShellResult::Err(ShellError::Error("cd: no such file or directory"))
        }
    } else {
        return ShellResult::Err(ShellError::Error("cd: to few arguments"))
    }
    ShellResult::Ok("".to_string())
}


#[cfg(test)]
mod test_cd {
    use super::*;

    #[test]
    fn test_up_down() {
        let cur_dir = env::current_dir().unwrap();
        let args = vec!["..".to_string()];
        cd(&args, Option::None);
        
        let new_args = vec![cur_dir.into_os_string().into_string().unwrap()];
        cd(&new_args, Option::None);

        assert_eq!(new_args[0], env::current_dir().unwrap().into_os_string().into_string().unwrap());
    }    
}
