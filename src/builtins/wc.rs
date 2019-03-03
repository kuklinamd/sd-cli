use super::common::get_content;
use crate::error::ShellError;
use crate::shell::ShellResult;

pub fn wc(args: &Vec<String>, msg: Option<String>) -> ShellResult<String> {
    if args.len() != 0 {
        let res = get_content(&args[0]);
        if let ShellResult::Ok(content) = res {
            return ShellResult::Ok(wc_inner(&content));
        } else {
            return res;
        }
    }
    if let Some(s) = msg {
        return ShellResult::Ok(wc_inner(&s));
    }
    ShellResult::Err(ShellError::Error("No input file or stream."))
}

fn wc_inner(content: &String) -> String {
    let chars = content.chars().count();
    let words = content.split_whitespace().count();
    let lines = content.lines().count();
    return format!("{} {} {}", lines, words, chars);
}
