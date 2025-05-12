use woosh::parser::shell::parse;

fn main() {
    let input = "echo hello world | sed 's/h/H/g' |grep foo > out.txt";
    match parse(input) {
        Ok(ast) => {
            println!("Parsed AST: {:#?}", ast);
        }
        Err(e) => {
            eprintln!("Parse error: {e:?}");
        }
    }
}
