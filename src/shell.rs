use std::io::{stdin, stdout, Write, BufRead};

use super::commands;
use super::environment::Env;
use super::parser;
use super::commands::ast_transform;
use super::builtins::Builtins;
use super::process;
use super::error::ShellError;

pub enum ShellResult<T> {
    Ok(T),
    Empty,
    Err(ShellError)
}

impl <T> ShellResult<T> {
    pub fn is_ok(&self) -> bool {
        match self {
            ShellResult::Ok(_) => true,
            _     => false
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            ShellResult::Empty => true,
            _     => false
        }
    }

    pub fn is_err(&self) -> bool {
        match self {
            ShellResult::Err(_) => true,
            _      => false
        }
    }
}

pub struct Shell {
    /// Shell's current local environment variables.
    local_env: Env,
    /// Shell's builtin routines.
    builtins: Builtins
}

impl Shell {
    /// Initialize shell with outter environment and default tables of builtin routines.
    pub fn init(env: Env) -> Shell {
        Shell { local_env: env, builtins: Builtins::init()}
    }

    /// Run main shell's loop.
    pub fn run(&mut self) {
        Shell::prompt("> ");
        for line in stdin().lock().lines() {
            match line {
                Ok(l) => self.handle(l),
                Err(e) => {
                    eprintln!("Cannot read line: {}", e);
                }
            }
            Shell::prompt("> ");
        }
    }

    /// Export variables to shell's local environment.
    pub fn export(&mut self, name: String, value: String) {
        self.local_env.add(name, value);
    }

    /// Execute command in shell's context.
    pub fn exec(&self, name: &String, args: &Vec<String>, msg: Option<String>) -> ShellResult<String> {
        if let Some(callback) = self.builtins.get(&name) {
            return callback.exec(args, msg)
        }
        return process::execute_external(name, args, msg)
    }

    fn prompt(p: &str) {
        print!("{}", p);
        // Need to flush stdout to see prompt.
        stdout().flush().expect("Could not flush stdout")
    }

    fn handle(&mut self, l: String) {
        if let Some(parsed) = parser::parse_cmd(l) {
            // Parsed AST -> Command AST
            let cmd = self.transform(&parsed);
            cmd.execute(self);
        } else {
            eprintln!("Unable to parse the command!");
        }
    }

    fn transform(&self, parsed: &parser::Command) -> commands::Command {
        ast_transform::ast_trans(&self.local_env, parsed)
    }

}
