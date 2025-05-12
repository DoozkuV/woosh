use anyhow::Result;
use woosh::{executor::eval, parser::parse};

fn main() -> Result<()> {
    let input = "grep rust | xargs";

    let ast = parse(input)?;
    eval(ast)?;
    Ok(())
}
