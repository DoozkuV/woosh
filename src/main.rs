use std::{env, process::exit};

use anyhow::Result;
use woosh::{executor::eval, parser::parse};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("Invalid number of arguments: {}", args.len());
        exit(1);
    }

    println!("{:?}", args);
    let input = args[1..].join(" ");

    let ast = parse(input.as_str())?;
    eval(ast).unwrap();
    Ok(())
}
