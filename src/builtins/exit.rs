use std::process;
use crate::shell::ShellResult;

/// Exit shell.
pub fn exit(_args: &Vec<String>, _msg: Option<String>) -> ShellResult<String> {
    process::exit(0);
}
