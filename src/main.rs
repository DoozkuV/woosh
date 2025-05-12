use woosh::parser::parse;

fn main() {
    print_parse("echo hello world | sed 's/h/H/g' |grep foo > out.txt");
    print_parse("echo 'good morning!'");
}

fn print_parse(input: &str) {
    match parse(input) {
        Ok(ast) => {
            println!("Parsed AST: {:#?}", ast);
        }
        Err(e) => {
            eprintln!("Parse error: {e:?}");
        }
    }
}
