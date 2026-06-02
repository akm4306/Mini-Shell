use rustyline::Editor;
use std::env;
use std::process::Command;

mod completer;
mod repl;

use completer::ShellCompleter;

fn spawn_in_new_window() -> bool {
    let exe = env::current_exe().expect("Failed to determine own executable path");
    let exe_str = exe.to_str().expect("Executable path is not valid UTF-8");

    let terminals: &[(&str, &[&str])] = &[
        ("kitty", &["--title", "Mini Shell", "-e"]),
        ("alacritty", &["--title", "Mini Shell", "-e"]),
        ("konsole", &["--new-mainwindow", "-e"]),
        ("gnome-terminal", &["--title", "Mini Shell", "--"]),
        ("xfce4-terminal", &["--title", "Mini Shell", "-e"]),
        ("xterm", &["-title", "Mini Shell", "-e"]),
    ];

    for (term, args) in terminals {
        if let Ok(mut child) = Command::new(term)
            .args(*args)
            .arg(exe_str)
            .env("MINI_SHELL_WINDOW", "1")
            .spawn()
        {
            let _ = child.wait();
            return true;
        }
    }

    false
}

fn main() {
    if env::var("MINI_SHELL_WINDOW").is_err() {
        if spawn_in_new_window() {
            return;
        }
        eprintln!("Warning: could not open a new terminal window; running in-place.");
    }

    let mut rl = Editor::new().unwrap();
    rl.set_helper(Some(ShellCompleter));
    let _ = rl.load_history(".mini_shell_history");

    loop {
        match repl::repl(&mut rl) {
            Ok(()) => continue,
            Err(_) => break,
        }
    }

    let _ = rl.save_history(".mini_shell_history");
}
