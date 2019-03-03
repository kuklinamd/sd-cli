use crate::error::ShellError;
use crate::shell::ShellResult;
pub fn echo(args: &Vec<String>, _msg: Option<String>) -> ShellResult<String> {
    ShellResult::Ok(args.join(" "))
}
