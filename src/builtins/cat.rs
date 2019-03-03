use super::common::get_content;
use crate::error::ShellError;
use crate::shell::ShellResult;

pub fn cat(args: &Vec<String>, msg: Option<String>) -> ShellResult<String> {
    if args.len() != 0 {
        return get_content(&args[0]);
    }
    if let Some(content) = msg {
        return ShellResult::Ok(content);
    }
    ShellResult::Err(ShellError::Error("No input file or stream"))
}
