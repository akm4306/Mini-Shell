use rustyline::Editor;

mod completer;
mod repl;

use completer::ShellCompleter;

fn main() {
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
