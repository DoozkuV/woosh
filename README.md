# Woosh

Woosh is a simple POSIX Shell implementation written in Rust to help me learn how shells work more deeply. Currently planned features include: 

- A basic AST Parser 
- A REPL 
- Evaluation of commands with POSIX features such as pipes
- Scripting support?
- Eventually a full implementation of the POSIX spec?


## Technical Specs

### Parser
Currently the parser is written using the `nom` Rust parser combinator library, which lets me easily implement my own parser. In the future, I may need to see about implementing some kind of custom error handling protocol as well as a more performant parser. However, I don't believe that most scripts require too much complexity regarding their parsing, especially with simple one-line REPL commands. Thus, for now I can get away with using something like Woosh as I flesh out the specification in more detail. 
