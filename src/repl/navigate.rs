pub fn pwd() -> String {
    return std::env::current_dir().unwrap().display().to_string();
}

pub fn cd(args: Vec<&str>) {
    if args.len() < 2 || args[1] == "~" {
        let path = std::env::var("HOME").unwrap();
        let chng = std::env::set_current_dir(&path);
        if chng.is_err() {
            println!("cd: {}: No such file or directory", &path);
        }
        return;
    }
    let path = args[1];
    let chng = std::env::set_current_dir(path);
    if chng.is_err() {
        println!("cd: {}: No such file or directory", path);
    }
}
