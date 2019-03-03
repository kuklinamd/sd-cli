use crate::error::ShellError;
pub fn echo(args: &Vec<String>, _msg: Option<String>) -> Result<String, ShellError> {
    Ok(args.join(" "))
}
