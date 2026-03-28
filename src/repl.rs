use std::io::{self, Write};
mod cat;
mod echo;
mod execute;
mod navigate;
mod parser;
mod type_;
pub fn repl() -> bool {
    print!("$ ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();
    let parsed_args = parser::parse_args(input.clone());
    let cmd = &parsed_args[0];
    if cmd == "exit" {
        return false;
    }
    if cmd == "echo" {
        echo::echo(input);
        return true;
    }
    if cmd == "type" {
        let args: Vec<&str> = parsed_args.iter().map(|s| s.as_str()).collect();
        type_::type_(args);
        return true;
    }
    if cmd == "pwd" {
        let wd = navigate::pwd();
        println!("{}", wd);
        return true;
    }
    if cmd == "cd" {
        let args: Vec<&str> = parsed_args.iter().map(|s| s.as_str()).collect();
        navigate::cd(args);
        return true;
    }
    if cmd == "cat" {
        cat::cat(input);
        return true;
    } else if type_::is_executable(cmd).is_some() {
        execute::execute(input);
    } else {
        println!("{}: command not found", cmd);
    }
    return true;
}
