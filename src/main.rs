#[allow(unused_imports)]
mod repl;

fn main() {
    let mut run = true;
    while run {
        run = repl::repl();
    }
}
