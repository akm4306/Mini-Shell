use std::io::Write;
use std::fs::OpenOptions;
use crate::repl::parser;

pub fn cat(input: String) {
    let args = parser::parse_args(input);
    if let Some(idx) = args.iter().position(|x| x == ">>" || x == "1>>") {
        let path = &args[idx + 1];
        let mut file = OpenOptions::new().append(true).create(true).open(path).unwrap();
        for arg in args[1..idx].iter() {
            let path = std::path::PathBuf::from(arg);
            if !path.exists() {
                println!("cat: {}: No such file or directory", arg);
                continue;
            }
            let contents = std::fs::read_to_string(path).expect("Failed to read file");
            write!(&mut file, "{}", contents).unwrap();
        }
    } else if let Some(idx) = args.iter().position(|x| x == "2>>") {
        let target_path = &args[idx + 1];
        let mut file = OpenOptions::new().append(true).create(true).open(target_path).unwrap();
        for arg in args[1..idx].iter() {
            let path = std::path::PathBuf::from(arg);
            if !path.exists() {
                write!(&mut file, "cat: {}: No such file or directory\n", arg).unwrap();
            } else {
                let contents = std::fs::read_to_string(path).expect("Failed to read file");
                print!("{}", contents);
            }
        }
    } else if let Some(idx) = args.iter().position(|x| x == ">" || x == "1>") {
        let path = &args[idx + 1];
        let mut file = std::fs::File::create(path).unwrap();
        for arg in args[1..idx].iter() {
            let path = std::path::PathBuf::from(arg);
            if !path.exists() {
                println!("cat: {}: No such file or directory", arg);
                continue;
            }
            let contents = std::fs::read_to_string(path).expect("Failed to read file");
            write!(&mut file, "{}", contents).unwrap();
        }
    } else if let Some(idx) = args.iter().position(|x| x == "2>") {
        let target_path = &args[idx + 1];
        let mut file = std::fs::File::create(target_path).unwrap();
        for arg in args[1..idx].iter() {
            let path = std::path::PathBuf::from(arg);
            if !path.exists() {
                write!(&mut file, "cat: {}: No such file or directory\n", arg).unwrap();
            } else {
                let contents = std::fs::read_to_string(path).expect("Failed to read file");
                print!("{}", contents);
            }
        }
    } else {
        for arg in args[1..].iter() {
            let path = std::path::PathBuf::from(arg);
            if !path.exists() {
                eprintln!("cat: {}: No such file or directory", arg);
            } else {
                let contents = std::fs::read_to_string(path).expect("Failed to read file");
                print!("{}", contents);
            }
        }
    }
}
