use crate::shell::ShellResult;

/// Print given arguments.
///
/// Doesn't read from pipe.
pub fn echo(args: &Vec<String>, _msg: Option<String>) -> ShellResult<String> {
    ShellResult::Ok(args.join(" "))
}
