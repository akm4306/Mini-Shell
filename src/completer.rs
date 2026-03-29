use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::{Context, Helper, Highlighter, Hinter, Validator};
use std::env;
use std::fs;
use std::path::Path;

const BUILTINS: &[&str] = &["exit", "echo", "type", "pwd", "cd", "cat"];


#[derive(Helper, Hinter, Highlighter, Validator)]
pub struct ShellCompleter;

impl Completer for ShellCompleter {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Pair>), ReadlineError> {
        let line_to_cursor = &line[..pos];

        let word_start = line_to_cursor.rfind(' ').map_or(0, |i| i + 1);
        let partial = &line_to_cursor[word_start..];

        let is_first_word = !line_to_cursor[..word_start].contains(' ')
            && word_start <= line_to_cursor.find(' ').unwrap_or(pos);

        let candidates = if is_first_word {
            complete_commands(partial)
        } else {
            complete_paths(partial)
        };

        Ok((word_start, candidates))
    }
}

fn complete_commands(partial: &str) -> Vec<Pair> {
    let mut candidates = Vec::new();

    for &cmd in BUILTINS {
        if cmd.starts_with(partial) {
            candidates.push(Pair {
                display: cmd.to_string(),
                replacement: cmd.to_string(),
            });
        }
    }

    if let Ok(path_var) = env::var("PATH") {
        for dir in path_var.split(':') {
            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let name = entry.file_name();
                    let name = name.to_string_lossy();
                    if name.starts_with(partial)
                        && !candidates.iter().any(|c: &Pair| c.display == *name)
                    {
                        candidates.push(Pair {
                            display: name.to_string(),
                            replacement: name.to_string(),
                        });
                    }
                }
            }
        }
    }

    candidates.sort_by(|a, b| a.display.cmp(&b.display));
    candidates
}

fn complete_paths(partial: &str) -> Vec<Pair> {
    let mut candidates = Vec::new();

    let (dir_to_search, file_prefix) = if partial.contains('/') {
        let p = Path::new(partial);
        let dir = p.parent().unwrap_or(Path::new("."));
        let prefix = p
            .file_name()
            .map(|f| f.to_string_lossy().to_string())
            .unwrap_or_default();
        (dir.to_path_buf(), prefix)
    } else {
        (std::path::PathBuf::from("."), partial.to_string())
    };

    if let Ok(entries) = fs::read_dir(&dir_to_search) {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy().to_string();
            if name_str.starts_with(&file_prefix) {
                let replacement = if partial.contains('/') {
                    let dir_prefix = &partial[..partial.rfind('/').unwrap() + 1];
                    if entry.path().is_dir() {
                        format!("{}{}/", dir_prefix, name_str)
                    } else {
                        format!("{}{}", dir_prefix, name_str)
                    }
                } else if entry.path().is_dir() {
                    format!("{}/", name_str)
                } else {
                    name_str.clone()
                };

                candidates.push(Pair {
                    display: name_str,
                    replacement,
                });
            }
        }
    }

    candidates.sort_by(|a, b| a.display.cmp(&b.display));
    candidates
}
