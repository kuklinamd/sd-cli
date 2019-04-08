use super::common::get_content;
use crate::error::ShellError;
use crate::shell::ShellResult;

/// Print content of the file given in the first argument.
/// If no argument given reads from pipe.
///
/// Doesn't read from stdin.
/// Doesn't actualy concatenate files.
pub fn cat(args: &Vec<String>, msg: Option<String>) -> ShellResult<String> {
    if args.len() != 0 {
        // It's okay if we get more than one argument. Just take the first.
        return get_content(&args[0]);
    }
    // Or try to read from pipe.
    if let Some(content) = msg {
        return ShellResult::Ok(content);
    }
    ShellResult::Err(ShellError::Error("No input file or stream"))
}
