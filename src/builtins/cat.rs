use std::io::{Read, BufReader};
use std::fs::File;

pub fn cat(args: &Vec<String>, msg: Option<String>) -> Option<String> {
    if args.len() != 0 {
        if let Ok(file) = File::open(&args[0]) {
            let mut buf  = BufReader::new(file);
            let mut content = String::new();
            if let Ok(_) = buf.read_to_string(&mut content) {
                return Some(content);
            } else {
                eprintln!("Couldn't read a file!");
            }
        } else {
            eprintln!("Couldn't open a file!");
        }
    } else {
        return msg;
    }

    None
}
