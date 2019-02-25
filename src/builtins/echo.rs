pub fn echo(args: &Vec<String>, _msg: Option<String>) -> Option<String> {
    Some(args.join(" "))
}
