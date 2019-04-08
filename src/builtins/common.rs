use std::io::{Read, BufReader};
use std::fs::File;

use crate::error::ShellError;
use crate::error::IOErr;

use crate::shell::ShellResult;

pub fn get_content(file: &String) -> ShellResult<String> {
    if let Ok(file) = File::open(&file) {
        let mut buf  = BufReader::new(file);
        let mut content = String::new();
        if let Ok(_) = buf.read_to_string(&mut content) {
            return ShellResult::Ok(content);
        } else {
            return ShellResult::Err(ShellError::IOError(IOErr::Read));
        }
    }
    ShellResult::Err(ShellError::IOError(IOErr::Open))
}
