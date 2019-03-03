use std::process;
use crate::error::ShellError;
pub fn exit(_args: &Vec<String>, _msg: Option<String>) -> Result<String, ShellError> {
    process::exit(0);
}
