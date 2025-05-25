pub const OPERATORS: [char; 2] = ['|', '>'];

#[derive(Debug)]
pub enum Command {
    Simple(SimpleCommand),
    Pipeline(Vec<SimpleCommand>),
    Empty,
}

#[derive(Debug)]
pub struct SimpleCommand {
    pub program: String,
    pub args: Vec<String>,
    pub redirection: Option<Redirection>,
}

#[derive(Debug)]
pub enum Redirection {
    Stdout(String), // > file
}
