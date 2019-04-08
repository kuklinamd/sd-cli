use crate::environment::Env;
use crate::parser as prsr;
use crate::commands as cmds;

/// Translate parsed AST to commands AST and interpolates
/// strings using environment.
///
/// We have two AST in the shell: Parsed AST and Command AST.
///
/// Parsed AST is the one we get from parser. It may contain
/// not interpolated strings.
///
/// Command AST is the one we traverse at execution stage.
/// It contains only interpolated strings (with all necessary
/// substitution of the environment variables).
pub fn ast_trans(env: &Env, parsed: &prsr::Command) -> cmds::Command {
    cmds::Command {
        cmd: trans_cmd(env, &parsed.get_cmd()),
        next: parsed.get_next().as_ref().map(|x| Box::new(trans_bind(env, x)))
    }
}

fn trans_bind(env: &Env, parsed: &prsr::Bind) -> cmds::Bind {
    cmds::Bind::Pipe(match parsed {
        prsr::Bind::Pipe(cmd) => ast_trans(env, cmd)
    })
}

fn trans_cmd(env: &Env, parsed: &prsr::CommandType) -> cmds::CommandType {
    match parsed {
        prsr::CommandType::Simple(cmd) => cmds::CommandType::Simple(trans_simpl(env, cmd)),
        prsr::CommandType::Env(exenv)  => cmds::CommandType::Env(trans_env(exenv))
    }
}

fn trans_env(parsed: &prsr::ExportedEnv) -> cmds::ExportedEnv {
    cmds::ExportedEnv { name: parsed.get_name().clone(), value: parsed.get_value().clone()}
}

fn trans_simpl(env: &Env, parsed: &prsr::SimpleCommand) -> cmds::SimpleCommand {
    cmds::SimpleCommand {
        name: trans_ex_name(env, parsed.get_name()),
        args: parsed.get_args().iter().map(|x| trans_ex_name(env, x)).collect()
    }
}

fn trans_ex_name(env: &Env, parsed: &prsr::ExtendedName) -> String {
    match parsed {
        prsr::ExtendedName::Name(n) => trans_name(env, n),
        prsr::ExtendedName::Extended(v) => trans_ex(env, v)
    }
}

fn trans_name(env: &Env, parsed: &prsr::Name) -> String {
    match parsed {
        prsr::Name::Plain(n) => n.to_string(),
        // if there's not such variable, return empty string.
        // default behaviour for bash/zsh.
        prsr::Name::EnvVar(n) => env.get(&n).unwrap_or("".to_string())
    }
}

fn trans_ex(env: &Env, parsed: &Vec<prsr::Name>) -> String {
    let mut strs = Vec::new();
    for name in parsed {
        strs.push(trans_name(env, name))
    }
    strs.join("")
}
