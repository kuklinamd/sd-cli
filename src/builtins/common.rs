use std::io::{Read, BufReader};
use std::fs::File;

use crate::error::ShellError;
use crate::error::IOErr;

pub fn get_content(file: &String) -> Result<String, ShellError> {
    if let Ok(file) = File::open(&file) {
        let mut buf  = BufReader::new(file);
        let mut content = String::new();
        if let Ok(_) = buf.read_to_string(&mut content) {
            return Ok(content);
        } else {
            return Err(ShellError::IOError(IOErr::Read));
        }
    }
    Err(ShellError::IOError(IOErr::Open))
}
