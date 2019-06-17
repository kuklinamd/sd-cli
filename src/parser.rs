/// `pest` is a parser crate.
extern crate pest;

use pest::Parser;
use pest_derive::Parser;

#[derive(PartialEq, Eq, Debug)]
pub enum Name {
    /// Name could be a simple string
    Plain(String),
    /// Or a environment variable.
    EnvVar(String),
}

#[derive(PartialEq, Eq, Debug)]
pub struct SimpleCommand {
    /// Name of the command.
    name: ExtendedName,
    /// Commands' arguments.
    args: Vec<ExtendedName>,
}
impl SimpleCommand {
    pub fn get_name(&self) -> &ExtendedName {
        &self.name
    }

    pub fn get_args(&self) -> &Vec<ExtendedName> {
        &self.args
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct ExportedEnv {
    /// Name of the variable.
    name: String,
    /// Value of the variable.
    value: String,
}
impl ExportedEnv {
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_value(&self) -> &String {
        &self.value
    }
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
    /// Not binded command.
    cmd: CommandType,
    /// Next binded command, if exists.
    next: Option<Box<Bind>>,
}
impl Command {
    pub fn get_cmd(&self) -> &CommandType {
        &self.cmd
    }

    pub fn get_next(&self) -> &Option<Box<Bind>> {
        &self.next
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum ExtendedName {
    /// Simple name: identifier, environment variable or single quoted string.
    Name(Name),
    /// Double quoted string, which could include environment variable,
    /// which we need to replace.
    Extended(Vec<Name>),
}

#[derive(Parser)]
// TODO: Is it a MAGICAL CONSTANT?
#[grammar = "grammar.pest"]
struct IdentParser;

fn translate_rule_envvar(pair: pest::iterators::Pair<Rule>) -> Name {
    let name = pair.into_inner().next().unwrap().as_str();
    Name::EnvVar(name.to_string())
}

fn translate_rule_envset(pair: pest::iterators::Pair<Rule>) -> ExportedEnv {
    let mut pairs = pair.into_inner();

    let name = pairs.next().unwrap().as_str();
    let val = pairs.next().unwrap().as_str();

    ExportedEnv {
        name: name.to_string(),
        value: val.to_string(),
    }
}

fn translate_rule_squote(pair: pest::iterators::Pair<Rule>) -> ExtendedName {
    ExtendedName::Name(Name::Plain(pair.as_str().to_string()))
}

fn translate_rule_dquote(pair: pest::iterators::Pair<Rule>) -> ExtendedName {
    let pairs = pair.into_inner();
    let mut extend = Vec::new();
    for inner_pair in pairs {
        let a = match inner_pair.as_rule() {
            Rule::envvar => translate_rule_envvar(inner_pair),
            Rule::ident_inside_dq_symb => Name::Plain(inner_pair.as_str().to_string()),
            x => {
                panic!(
                    "Must not happen. Check grammar file. Error: {:?} {}",
                    x,
                    inner_pair.as_str()
                )
            }
        };
        extend.push(a);
    }
    ExtendedName::Extended(extend)
}

fn translate_rule_envvars(envs: Vec<pest::iterators::Pair<Rule>>) -> ExtendedName {
    //let exname = ExtendedName::Name(translate_rule_envvar(pair));
    let mut exnames = Vec::new();
    for env in envs {
        exnames.push(translate_rule_envvar(env));
    }
    ExtendedName::Extended(exnames)
}

fn translate_rule_name(pair: pest::iterators::Pair<Rule>) -> ExtendedName {
    let mut pairs = pair.into_inner();
    let inner_pair = pairs.next().unwrap();
    match inner_pair.as_rule() {
        Rule::ident => ExtendedName::Name(Name::Plain(inner_pair.as_str().to_string())),
        Rule::envvar => {
            let mut envs = Vec::new();
            envs.push(inner_pair);
            while let Some(peek_pair) = pairs.peek() {
                if peek_pair.as_rule() == Rule::envvar {
                    let p = pairs.next().unwrap();
                    envs.push(p);
                } else {
                    break;
                }
            }
            translate_rule_envvars(envs)
        },
        Rule::ident_inside_dq => translate_rule_dquote(inner_pair),
        Rule::ident_inside_sq => translate_rule_squote(inner_pair),
        x => {
            panic!(
                "Must not happen. Check grammar file. Error: {:?} {}",
                x,
                inner_pair.as_str()
            )
        }
    }
}

fn translate_rule_option(pair: pest::iterators::Pair<Rule>) -> ExtendedName {
    let pairs = pair.into_inner().next().unwrap();
    translate_rule_name(pairs)
}

fn translate_rule_single_cmd(pair: pest::iterators::Pair<Rule>) -> SimpleCommand {
    let mut pairs = pair.into_inner();
    let name = translate_rule_name(pairs.next().unwrap());
    let mut opts = Vec::new();
    for inner_pair in pairs {
        match inner_pair.as_rule() {
            Rule::option => {
                let opt = translate_rule_option(inner_pair);
                opts.push(opt);
            }
            x => {
                panic!(
                    "Must not happen. Check grammar file. Error: {:?} {}",
                    x,
                    inner_pair.as_str()
                )
            }
        }
    }
    SimpleCommand {
        name: name,
        args: opts,
    }
}

/// Translate inner `pest` representation of a command into our one.
fn translate_rule_cmd(pair: pest::iterators::Pair<Rule>) -> Command {
    let mut pairs = pair.into_inner();
    let inner_pair = pairs.next().unwrap();
    let cmd = match inner_pair.as_rule() {
        Rule::single_cmd => CommandType::Simple(translate_rule_single_cmd(inner_pair)),
        Rule::envset => CommandType::Env(translate_rule_envset(inner_pair)),
        x => {
            panic!(
                "Must not happen. Check grammar file. Error: {:?} {}",
                x,
                inner_pair.as_str()
            )
        }
    };

    let next_pair = pairs.next().unwrap();
    let next_cmd = match next_pair.as_rule() {
        Rule::cmd => Some(Box::new(Bind::Pipe(translate_rule_cmd(next_pair)))),
        Rule::EOI => None,
        x => panic!("Must not happen: {:?} {}", x, next_pair.as_str()),
    };

    Command {
        cmd: cmd,
        next: next_cmd,
    }
}

/// Parse string command into inner representation of a command structure.
///
/// TODO: We consider that `unwrap()` is save if `parse` returns
/// `Ok(_)`, because if parsing failes `Err()` returns.
///
pub fn parse_cmd(cmd: String) -> Option<Command> {
    match IdentParser::parse(Rule::cmd, cmd.as_str()) {
        Ok(mut pairs) => Some(translate_rule_cmd(pairs.next().unwrap())),
        _ => None,
    }
}


#[cfg(test)]
mod test_parser {
    use super::*;

    use super::ExtendedName::*;
    use super::Name::*;

    #[test]
    fn test_simple() {
        let tcmd = parse_cmd("echo 1 2".to_string()).unwrap();
        let dcmd = Command {
            cmd: CommandType::Simple(SimpleCommand {
                name: Name(Plain("echo".to_string())),
                args: vec![Name(Plain("1".to_string())), Name(Plain("2".to_string()))],
            }),
            next: None,
        };
        assert_eq!(tcmd, dcmd);
    }

    #[test]
    fn test_simple_env() {
        let tcmd = parse_cmd("echo $var".to_string()).unwrap();
        let dcmd = Command {
            cmd: CommandType::Simple(SimpleCommand {
                name: Name(Plain("echo".to_string())),
                args: vec![Name(EnvVar("var".to_string()))],
            }),
            next: None,
        };
        assert_eq!(tcmd, dcmd);
    }

    #[test]
    fn test_simpl_sq() {
        let tcmd = parse_cmd("echo '$var'".to_string()).unwrap();
        let dcmd = Command {
            cmd: CommandType::Simple(SimpleCommand {
                name: Name(Plain("echo".to_string())),
                args: vec![Name(Plain("$var".to_string()))],
            }),
            next: None,
        };
        assert_eq!(tcmd, dcmd);
    }

    #[test]
    fn test_simpl_dq() {
        let tcmd = parse_cmd("echo \"$var\"".to_string()).unwrap();
        let dcmd = Command {
            cmd: CommandType::Simple(SimpleCommand {
                name: Name(Plain("echo".to_string())),
                args: vec![Extended(vec![EnvVar("var".to_string())])],
            }),
            next: None,
        };
        assert_eq!(tcmd, dcmd);
    }

    #[test]
    fn test_env() {
        let tcmd = parse_cmd("var=10".to_string()).unwrap();
        let dcmd = Command {
            cmd: CommandType::Env(ExportedEnv {
                name: "var".to_string(),
                value: "10".to_string(),
            }),
            next: None,
        };
        assert_eq!(tcmd, dcmd);
    }

    #[test]
    fn test_pipe() {
        let tcmd = parse_cmd("echo 1 2 | wc".to_string()).unwrap();
        let dcmd = Command {
            cmd: CommandType::Simple(SimpleCommand {
                name: Name(Plain("echo".to_string())),
                args: vec![Name(Plain("1".to_string())), Name(Plain("2".to_string()))],
            }),
            next: Some(Box::new(Bind::Pipe(Command {
                cmd: CommandType::Simple(SimpleCommand {
                    name: Name(Plain("wc".to_string())),
                    args: vec![],
                }),
                next: None,
            }))),
        };
        assert_eq!(tcmd, dcmd);
    }
}
