pub mod wc;
pub mod echo;
pub mod pwd;
pub mod exit;
pub mod cat;

mod common;

use std::collections::HashMap;

use crate::builtins;

use super::shell::ShellResult;

/// Dictionary of builtin functions.
pub struct Builtins(HashMap<String, Builtin>);
impl Builtins {
    pub fn new() -> Builtins {
        Builtins(HashMap::new())
    }

    pub fn init() -> Builtins {
        // Create list of builtin commands.
        let mut b = Builtins::new();
        b.add("echo", Builtin(Box::new(builtins::echo::echo)));
        b.add("wc",   Builtin(Box::new(builtins::wc::wc)));
        b.add("pwd",  Builtin(Box::new(builtins::pwd::pwd)));
        b.add("exit", Builtin(Box::new(builtins::exit::exit)));
        b.add("cat",  Builtin(Box::new(builtins::cat::cat)));
        b
    }

    pub fn add(&mut self, name: &str, callback: Builtin) {
        self.0.insert(name.to_string(), callback);
    }

    pub fn get(&self, name: &str) -> Option<&Builtin> {
        self.0.get(name)
    }
}

/// Pointer to builtin function.
///
/// `builtin` takes:
///  * `&Vec<String>` -- command line arguments.
///  * Option<String> -- input to stdin.
///
pub struct Builtin(Box<Fn(&Vec<String>, Option<String>) -> ShellResult<String>>);
impl Builtin {
    pub fn new(callback: Box<Fn(&Vec<String>, Option<String>) -> ShellResult<String>>) -> Builtin {
        Builtin(callback)
    }

    pub fn exec(&self, args: &Vec<String>, msg: Option<String>) -> ShellResult<String> {
        (*self.0)(args, msg)
    }
}
