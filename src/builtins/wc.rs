use super::common::get_content;
use crate::error::ShellError;

pub fn wc(args: &Vec<String>, msg: Option<String>) -> Result<String, ShellError> {
    if args.len() != 0 {
        let res = get_content(&args[0]);
        if let Ok(content) = res {
            return Ok(wc_inner(&content));
        } else {
            return res;
        }
    }
    if let Some(s) = msg {
        return Ok(wc_inner(&s));
    }
    Err(ShellError::Error("No input file or stream."))
}

fn wc_inner(content: &String) -> String {
    let chars = content.chars().count();
    let words = content.split_whitespace().count();
    let lines = content.lines().count();
    return format!("{} {} {}", lines, words, chars);
}
