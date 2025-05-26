use anyhow::Result;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

use crate::executor::eval;
use crate::parser;

/// Starts the interactive repl
pub fn start_interactive() -> Result<()> {
    let mut rl = DefaultEditor::new()?;

    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                let trimmed = line.trim_end();
                if !trimmed.is_empty() {
                    rl.add_history_entry(trimmed)?;
                }

                let ast = parser::parse_line(trimmed)?;
                let _ = eval(ast);
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                continue;
            }
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                // TODO: Properly input error code here
                eprintln!("Error reading line: {err}");
                break;
            }
        }
    }

    Ok(())
}
