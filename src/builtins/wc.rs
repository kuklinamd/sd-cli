use super::common::get_content;
use crate::error::ShellError;
use crate::shell::ShellResult;

/// Count number of symbols, words and lines in given file or pipe.
pub fn wc(args: &Vec<String>, msg: Option<String>) -> ShellResult<String> {
    if args.len() != 0 {
        // It's ok if we get more than one argument.
        // Just take the first one.
        let res = get_content(&args[0]);
        if let ShellResult::Ok(content) = res {
            return ShellResult::Ok(wc_inner(&content));
        } else {
            // If `res` isn't Ok, it contains valid error.
            return res;
        }
    }
    // If there's no argument, try to "read" from "pipe".
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
