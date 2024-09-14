use std::io::{self, Write};

use monkey_interpreter::{lexer::Lexer, parser::Parser};

fn main() {
    loop {
        print!("->");
        io::stdout().flush().unwrap();
    
        let mut input = String::new();
    
        io::stdin().read_line(&mut input).expect("Failed to read line");
    
        let input = input.trim();
    
        let lexer = Lexer::new(input.to_string());

        let mut parser = Parser::new(lexer);
    
        // let mut token = lexer.next_token();

        match parser.parse_program() {
            Ok(program) => {
                for statement in &program.statements {
                    println!("{}", statement.dbg());
                }
    
                println!("{program:#?}")
            },
            Err(err) => println!("{err:?}")
        }
    }
}
