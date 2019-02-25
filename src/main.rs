pub mod commands;
pub mod parser;
pub mod environment;
pub mod shell;
pub mod builtins;
pub mod process;

use shell::Shell;
use environment::Env;

fn main() {
    Shell::init(Env::new()).run();
}
