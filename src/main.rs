use anyhow::{Ok, Result};
use clap::Parser;
use woosh::executor::eval;
use woosh::parser::parse_line;
use woosh::repl::start_interactive;

#[derive(Parser, Debug)]
struct Args {
    /// Execute a one-liner command
    #[arg(short = 'c', long = "command")]
    command: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if let Some(cmd) = args.command {
        let ast = parse_line(cmd.as_str())?;
        eval(ast)?;
    } else {
        start_interactive()?;
    }
    Ok(())
}
