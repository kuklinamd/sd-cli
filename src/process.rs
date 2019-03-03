use std::io::{Write};
use std::process::{Command, Stdio};

use super::error::{ShellError,IOErr};
use super::shell::ShellResult;

/// Execute system command with given `name`. Passes `args` as command-line
/// arguments and sends `msg` as input, if the command is the part of pipeline.
///
/// *Returns* result if is exists.
pub fn execute_external(name: &String, args: &Vec<String>, msg: Option<String>) -> ShellResult<String> {
    let cmd = Command::new(name)
              .args(args)
              .stdin(Stdio::piped())
              .stdout(Stdio::piped())
              .spawn();

    if let Ok(mut cmd) = cmd {
        if let Some(m) = msg {
            // Is `msg` isn't None, we want to send its content
            // to stdin of the executed command.
            if let Some(ref mut sin) = (&mut cmd).stdin {
                if let Err(_) = sin.write_all(m.as_bytes()) {
                    return ShellResult::Err(ShellError::Error("Unable to write to the process stdin."));
                }
            }
        }

        // Wait from command output to send it either next through
        // pipeline or to stdout.
        if let Ok(out) = cmd.wait_with_output() {
            if let Ok(s) = String::from_utf8(out.stdout) {
                return ShellResult::Ok(s);
            }
            return ShellResult::Err(ShellError::IOError(IOErr::Read));
        }
    }
    ShellResult::Err(ShellError::IOError(IOErr::Exec))
}
