use std::collections::HashSet;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

pub struct BuiltIn {
    kw: HashSet<String>,
}

impl BuiltIn {
    pub fn new() -> BuiltIn {
        let set = HashSet::from([
            "echo".to_string(),
            "type".to_string(),
            "exit".to_string(),
            "pwd".to_string(),
            "cd".to_string(),
        ]);
        BuiltIn { kw: set }
    }
    pub fn is_built_in(&self, s: &str) -> bool {
        return self.kw.contains(s);
    }
}

pub fn is_executable(arg: &str) -> Option<PathBuf> {
    let path_var = std::env::var("PATH").unwrap_or_default();
    for path in std::env::split_paths(&path_var) {
        let full_path = path.join(arg);
        if let Ok(metadata) = std::fs::metadata(&full_path) {
            if metadata.is_file() && metadata.permissions().mode() & 0o111 != 0 {
                return Some(full_path);
            }
        }
    }
    None
}

pub fn type_(args: Vec<&str>) {
    let built_in = BuiltIn::new();
    if args.len() < 2 {
        return;
    }

    let cmd = args[1];
    if built_in.is_built_in(cmd) {
        println!("{} is a shell builtin", cmd);
    } else if let Some(path) = is_executable(cmd) {
        println!("{} is {}", cmd, path.display());
    } else {
        println!("{}: not found", cmd);
    }
}
