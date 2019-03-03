use std::process;
use crate::error::ShellError;
use crate::shell::ShellResult;

pub fn exit(_args: &Vec<String>, _msg: Option<String>) -> ShellResult<String> {
    process::exit(0);
}
