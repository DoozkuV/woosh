use std::io::{stdin, stdout, Write};

use anyhow::Result;

use crate::{executor::eval, parser};

/// Starts the interactive repl
pub fn start_interactive() -> Result<()> {
    let stdin = stdin();
    let mut stdout = stdout();
    let mut buffer = String::new();

    loop {
        print!("> ");
        stdout.flush()?;

        buffer.clear();
        let bytes_read = stdin.read_line(&mut buffer)?;

        // Ctrl+D
        if bytes_read == 0 {
            return Ok(());
        }

        let ast = parser::parse_line(&buffer)?;
        let _ = eval(ast);
    }
}
