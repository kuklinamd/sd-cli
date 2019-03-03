use std::ops::Deref;

use super::shell::Shell;
use crate::error;
use crate::error::ShellError;

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
            CommandType::Env(ref e) => {
                Command::export(&e, shell);
                Ok("".to_string())
            },
            CommandType::Simple(ref c) => Command::exec(&c, shell, msg)
        };

        if let Some(ref bind) = self.next {
            // Well, now we have only one Bind pattern.
            let Bind::Pipe(c) = (*bind).deref();

            if let Ok(s) = stdio {
                c.execute_rec(shell, Some(s));
            } else {
                error::eprint(stdio.err().unwrap());
                c.execute_rec(shell, None);
            }
        } else {
            if let Ok(s) = stdio {
                println!("{}", s);
            } else {
                error::eprint(stdio.err().unwrap());
            }
        }
    }

    // Export variable inside shell's evironment.
    fn export(env: &ExportedEnv, shell: &mut Shell) {
        shell.export(env.name.clone(), env.value.clone());
    }

    // Execute command inside shell's environment.
    fn exec(cmd: &SimpleCommand, shell: &Shell, msg: Option<String>) -> Result<String, ShellError> {
        shell.exec(&cmd.name, &cmd.args, msg)
    }
}
