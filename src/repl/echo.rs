use std::io::Write;
use std::fs::OpenOptions;
use crate::repl::parser;

pub fn echo(input: String) {
    let args = parser::parse_args(input);
    if let Some(idx) = args.iter().position(|x| x == ">>" || x == "1>>") {
        let path = &args[idx + 1];
        let mut file = OpenOptions::new().append(true).create(true).open(path).unwrap();
        for arg in args[1..idx - 1].iter() {
            write!(&mut file, "{} ", arg).unwrap();
        }
        write!(&mut file, "{}\n", args[idx - 1]).unwrap();
    } else if let Some(idx) = args.iter().position(|x| x == "2>>") {
        let path = &args[idx + 1];
        let _file = OpenOptions::new().append(true).create(true).open(path).unwrap();
        for arg in args[1..idx].iter() {
            print!("{} ", arg);
        }
        println!();
    } else if let Some(idx) = args.iter().position(|x| x == ">" || x == "1>") {
        let path = &args[idx + 1];
        let mut file = std::fs::File::create(path).unwrap();
        for arg in args[1..idx - 1].iter() {
            write!(&mut file, "{} ", arg).unwrap();
        }
        write!(&mut file, "{}\n", args[idx - 1]).unwrap();
    } else if let Some(idx) = args.iter().position(|x| x == "2>") {
        let path = &args[idx + 1];
        let _file = std::fs::File::create(path).unwrap();
        for arg in args[1..idx].iter() {
            print!("{} ", arg);
        }
        println!();
    } else {
        for arg in args[1..].iter() {
            print!("{} ", arg);
        }
        println!();
    }
}
