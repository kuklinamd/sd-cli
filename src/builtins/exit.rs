use std::process;

pub fn exit(_args: &Vec<String>, _msg: Option<String>) -> Option<String> {
    process::exit(0);
}
