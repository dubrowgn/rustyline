extern crate tcalc_rustyline;

use tcalc_rustyline::completion::FilenameCompleter;
use tcalc_rustyline::error::ReadlineError;
use tcalc_rustyline::{Config, CompletionType, Editor};

// On unix platforms you can use ANSI escape sequences
#[cfg(unix)]
static PROMPT: &'static str = "\x1b[1;32m>>\x1b[0m ";

// Windows consoles typically don't support ANSI escape sequences out
// of the box
#[cfg(windows)]
static PROMPT: &'static str = ">> ";

fn main() {
    let config = Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .build();
    let c = FilenameCompleter::new();
    let mut rl = Editor::with_config(config);
    rl.set_completer(Some(c));
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(PROMPT);
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_ref());
                println!("Line: {}", line);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("history.txt").unwrap();
}
