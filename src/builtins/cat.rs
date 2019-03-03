use super::common::get_content;
use crate::error::ShellError;

pub fn cat(args: &Vec<String>, msg: Option<String>) -> Result<String, ShellError> {
    if args.len() != 0 {
        return get_content(&args[0]);
    }
    if let Some(content) = msg {
        return Ok(content);
    }
    Err(ShellError::Error("No input file or stream"))
}
