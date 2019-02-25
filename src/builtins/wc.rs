use std::io::{Read, BufReader};
use std::fs::File;

pub fn wc(args: &Vec<String>, msg: Option<String>) -> Option<String> {
    if args.len() != 0 {
        if let Ok(file) = File::open(&args[0]) {
            let mut buf  = BufReader::new(file);
            let mut content = String::new();
            if let Ok(_) = buf.read_to_string(&mut content) {
                return Some(wc_inner(&content));
            } else {
                eprintln!("Couldn't read a file!");
            }
        } else {
            eprintln!("Couldn't open a file!");
        }
    } else {
        if let Some(s) = msg {
            return Some(wc_inner(&s));
        } else {
            eprintln!("No file or stream.");
        }
    }

    None
}

fn wc_inner(content: &String) -> String {
    let chars = content.chars().count();
    let words = content.split_whitespace().count();
    let lines = content.lines().count();
    return format!("{} {} {}", lines, words, chars);
}
