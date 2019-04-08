use std::ops::Deref;

use super::shell::{Shell,ShellResult};
use crate::error;

/// Builtin or system command.
#[derive(PartialEq, Eq, Debug)]
pub struct SimpleCommand {
    name: String,
    args: Vec<String>
}

/// Set environment variable `name` with `value`.
#[derive(PartialEq, Eq, Debug)]
pub struct ExportedEnv {
    name: String,
    value: String
}

#[derive(PartialEq, Eq, Debug)]
pub enum CommandType {
    /// Execute a simple command (could contain environment variables).
    Simple(SimpleCommand),
    /// Export environment variable.
    Env(ExportedEnv),
}

#[derive(PartialEq, Eq, Debug)]
pub enum Bind {
    /// Bind two commands with pipe.
    Pipe(Command),
}

#[derive(PartialEq, Eq, Debug)]
pub struct Command {
    /// Command itself.
    cmd: CommandType,
    /// Next binded command, if exists.
    next: Option<Box<Bind>>,
}

// Inner module.
pub mod ast_transform;

impl Command {
    pub fn execute(&self, shell: &mut Shell) {
        self.execute_rec(shell, None);
    }

    // Recursively traverse command's AST and execute commands at nodes.
    fn execute_rec(&self, shell: &mut Shell, msg: Option<String>) {
        let stdio = match self.cmd {
            CommandType::Env(ref e) => Command::export(&e, shell),
            CommandType::Simple(ref c) => Command::exec(&c, shell, msg)
        };

        if let Some(ref bind) = self.next {
            // Well, now we have only one Bind pattern.
            let Bind::Pipe(c) = (*bind).deref();
            match stdio {
                ShellResult::Ok(s) => c.execute_rec(shell, Some(s)),
                ShellResult::Empty => c.execute_rec(shell, None),
                ShellResult::Err(e) => {
                    error::eprint(e);
                    c.execute_rec(shell, None)
                }
            };
        } else {
            match stdio {
                ShellResult::Ok(s)  => println!("{}", s),
                ShellResult::Err(e) => error::eprint(e),
                ShellResult::Empty  => ()
            };
        }
    }

    // Export variable inside shell's evironment.
    fn export(env: &ExportedEnv, shell: &mut Shell) -> ShellResult<String> {
        shell.export(env.name.clone(), env.value.clone());
        ShellResult::Empty
    }

    // Execute command inside shell's environment.
    fn exec(cmd: &SimpleCommand, shell: &Shell, msg: Option<String>) -> ShellResult<String> {
        shell.exec(&cmd.name, &cmd.args, msg)
    }
}
