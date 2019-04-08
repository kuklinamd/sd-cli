/// `getopts` is a command line parsing library.
extern crate getopts;
/// `regex` is a regex library.
extern crate regex;

use getopts::Options;
use regex::{Regex,RegexBuilder};

use crate::error::{ShellError, IOErr};
use crate::shell::ShellResult;
use super::common::get_content;


#[derive(Debug)]
/// Modes of `grep` command.
struct GrepMode {
    /// Ignore case flag.
    is_ignore: bool,
    /// 'Search as a word' flag.
    is_word: bool,
    /// 'Append' flag.
    /// Some(n) -- append flag is given and `n` is number of lines to append.
    /// None -- no append flag.
    is_append: Option<u64>
}

/// Format: grep [OPTIONS] [FILE]
/// * Options:
///     * -i -- ignore case
///     * -w -- search as a word
///     * -A n -- print `n` additional lines after the matched line
pub fn grep (args: &Vec<String>, msg: Option<String>) -> ShellResult<String> {
    let mut opts = Options::new();
    opts.optflag("w", "", "search as a word");
    opts.optflag("i", "", "ignore case");
    opts.optopt("A", "", "print `n` additional lines after the matched line", "N");

    let matches = match opts.parse(args) {
        Ok(m) => m,
        Err(_) => {
            return ShellResult::Err(ShellError::Error("Unable to parse command line arguments."));
        }
    };

    // `free` contains the rest non-options arguments.
    if matches.free.is_empty() {
        return ShellResult::Err(ShellError::Error("No pattern to grep."))
    }

    let pattern = matches.free[0].clone();

    let append_arg = if matches.opt_present("A") {
        match matches.opt_str("A") {
            Some(num) => {
                if let Ok(n) = num.parse::<u64>() {
                    Some(n + 1)
                } else {
                    return ShellResult::Err(ShellError::Error("Option `A` needs a number as an argument."))
                }
            },
            None => return ShellResult::Err(ShellError::Error("Option `A` needs a number as an argument."))
        }
    } else { None };

    let mode = GrepMode {
        is_ignore: matches.opt_present("i"),
        is_word: matches.opt_present("w"),
        is_append: append_arg
    };

    if matches.free.len() < 2 {
        if let Some(content) = msg {
            return grep_impl(&content, pattern, &mode);
        } else {
            return ShellResult::Err(ShellError::IOError(IOErr::Read))
        }
    }
    let file = &matches.free[1];
    let res: ShellResult<_> = get_content(file);
    if let ShellResult::Ok(content) = res {
        return grep_impl(&content, pattern, &mode);
    }
    res
}

fn build_regex(pattern: String, mode: &GrepMode) -> Option<Regex> {
    let pat = if mode.is_word { format!("\\b{}\\b", pattern).to_string() }
              else { pattern };

    RegexBuilder::new(&pat).case_insensitive(mode.is_ignore).build().ok()
}

fn grep_impl(content: &String, pattern: String, mode: &GrepMode) -> ShellResult<String> {

    let regex = build_regex(pattern, mode);
    let regex = match regex {
        Some(r) => r,
        None    => return ShellResult::Err(ShellError::Error("Unable to build regex."))
    };

    let mut allow_to_print = 0;
    let mut output = Vec::new();
    for line in content.lines() {
        if allow_to_print > 0 {
            output.push(line.clone());
            allow_to_print -= 1;
        } else {
            let res = regex.find(line);
            if let Some(_) = res {
                output.push(line.clone());
                if let Some(a) = mode.is_append {
                    allow_to_print = a;
                }
            }
        }
    }
    ShellResult::Ok(output.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const test_str: &'static str = "test string for testing\nbut it is not\nHe's not a Tesla\nThis Test for us\ntesting";

    #[test]
    fn test_grep_nothing() {
        let a = grep(&vec![], None);
        assert!(a.is_err());
    }

    #[test]
    fn test_grep_no_match() {
        let a = grep(&vec!["x".to_string()], Some("y".to_string()));
        assert_eq!(a, ShellResult::Ok("".to_string()));
    }

    #[test]
    fn test_grep_ok() {
        let a = grep(&vec!["test".to_string()], Some("test string for testing".to_string()));
        assert!(a.is_ok());
    }

    #[test]
    fn test_grep_impl_simpl() {
        let mode = GrepMode { is_ignore: false, is_word: false, is_append: None};
        let res = grep_impl(&test_str.to_string(), "test".to_string(), &mode);
        match res {
            ShellResult::Ok(s) => assert!(s == "test string for testing\ntesting"),
            _ => assert!(false)
        };
    }

    #[test]
    fn test_grep_impl_append() {
        let mode = GrepMode { is_ignore: false, is_word: false, is_append: Some(1)};
        let res = grep_impl(&test_str.to_string(), "test".to_string(), &mode);
        match res {
            ShellResult::Ok(s) => assert!(s == "test string for testing\nbut it is not\ntesting"),
            _ => assert!(false)
        };
    }

    #[test]
    fn test_grep_impl_ignore() {
        let mode = GrepMode { is_ignore: true, is_word: false, is_append: None};
        let res = grep_impl(&test_str.to_string(), "test".to_string(), &mode);
        match res {
            ShellResult::Ok(s) => assert!(s == "test string for testing\nThis Test for us\ntesting"),
            _ => assert!(false)
        };
    }

    #[test]
    fn test_grep_impl_word() {
        let mode = GrepMode { is_ignore: false, is_word: true, is_append: None};
        let res = grep_impl(&test_str.to_string(), "test".to_string(), &mode);
        match res {
            ShellResult::Ok(s) => assert!(s == "test string for testing"),
            _ => assert!(false)
        };
    }
}
