mod cat;
mod echo;
mod execute;
mod navigate;
mod parser;
mod type_;

use crate::completer::ShellCompleter;
use rustyline::Editor;

pub fn repl(
    rl: &mut Editor<ShellCompleter, rustyline::history::DefaultHistory>,
) -> rustyline::Result<()> {
    let input = rl.readline("$ ")?;
    rl.add_history_entry(&input)?;

    let parsed_args = parser::parse_args(input.clone());
    let cmd = &parsed_args[0];
    if cmd == "exit" {
        return Err(rustyline::error::ReadlineError::Interrupted);
    } else if cmd == "echo" {
        echo::echo(input.clone());
    } else if cmd == "type" {
        let args: Vec<&str> = parsed_args.iter().map(|s| s.as_str()).collect();
        type_::type_(args);
    } else if cmd == "pwd" {
        let wd = navigate::pwd();
        println!("{}", wd);
    } else if cmd == "cd" {
        let args: Vec<&str> = parsed_args.iter().map(|s| s.as_str()).collect();
        navigate::cd(args);
    } else if cmd == "cat" {
        cat::cat(input.clone());
    } else if type_::is_executable(cmd).is_some() {
        execute::execute(input.clone());
    } else {
        println!("{}: command not found", cmd);
    }
    Ok(())
}
