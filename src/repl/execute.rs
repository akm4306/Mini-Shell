use std::io::Read;
use std::process::{Command, Stdio};
use std::fs::OpenOptions;

use super::parser;
use super::type_::is_executable;

pub fn execute(input: String) {
    let args = parser::parse_args(input);
    if let Some(idx) = args.iter().position(|x| (x == ">>") || (x == "1>>")) {
        let target_path = &args[idx + 1];
        let file = OpenOptions::new().append(true).create(true).open(target_path).unwrap();
        let mut child = Command::new(&args[0])
            .args(&args[1..idx])
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to spawn");
        let std_out = child.stdout.take().unwrap();
        let mut reader = std::io::BufReader::new(std_out);
        let mut writer = std::io::BufWriter::new(file);
        std::io::copy(&mut reader, &mut writer).unwrap();
    } else if let Some(idx) = args.iter().position(|x| x == "2>>") {
        let target_path = &args[idx + 1];
        let file = OpenOptions::new().append(true).create(true).open(target_path).unwrap();
        let mut child = Command::new(&args[0])
            .args(&args[1..idx])
            .stdout(Stdio::inherit())
            .stderr(Stdio::piped())
            .spawn()
            .expect("failed to spawn");
        let std_err = child.stderr.take().unwrap();
        let mut reader = std::io::BufReader::new(std_err);
        let mut writer = std::io::BufWriter::new(file);
        std::io::copy(&mut reader, &mut writer).unwrap();
        drop(writer);
        child.wait().expect("failed to wait on child");
    } else if let Some(idx) = args.iter().position(|x| (x == ">") || (x == "1>")) {
        let target_path = &args[idx + 1];
        let file = std::fs::File::create(target_path).unwrap();
        let mut child = Command::new(&args[0])
            .args(&args[1..idx])
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to spawn");
        let std_out = child.stdout.take().unwrap();
        let mut reader = std::io::BufReader::new(std_out);
        let mut writer = std::io::BufWriter::new(file);
        std::io::copy(&mut reader, &mut writer).unwrap();
    } else if let Some(idx) = args.iter().position(|x| x == "2>") {
        let target_path = &args[idx + 1];
        let file = std::fs::File::create(target_path).unwrap();
        let mut child = Command::new(&args[0])
            .args(&args[1..idx])
            .stdout(Stdio::inherit())
            .stderr(Stdio::piped())
            .spawn()
            .expect("failed to spawn");
        let std_err = child.stderr.take().unwrap();
        let mut reader = std::io::BufReader::new(std_err);
        let mut writer = std::io::BufWriter::new(file);
        std::io::copy(&mut reader, &mut writer).unwrap();
        drop(writer);
        child.wait().expect("failed to wait on child");
    } else {
        let mut child = Command::new(&args[0])
            .args(&args[1..])
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to spawn");
        let std_out = child.stdout.take().unwrap();
        let mut reader = std::io::BufReader::new(std_out);
        let mut output = String::new();
        reader.read_to_string(&mut output).unwrap();
        print!("{}", output);
    }
}
