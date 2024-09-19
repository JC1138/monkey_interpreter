use clap::Parser;
use monkey_interpreter::interpreter::{Environment, Interpreter};
use monkey_interpreter::parser::Program;
use std::fs;
use std::path::Path;

use std::io::{self, Write};

use monkey_interpreter::{lexer::Lexer, parser};
#[derive(Parser)]
struct Args {
    /// The file name to read (located in /programs directory)
    #[arg(long)]
    file: Option<String>,

    #[arg(long)]
    filee: Option<String>,

    #[arg(long, action = clap::ArgAction::SetTrue)]
    repl: bool,

    #[arg(long, action = clap::ArgAction::SetTrue)]
    reple: bool,
}

fn main() -> Result<(), std::io::Error> {
    
    let args = Args::parse();

    if args.repl {
        start_repl(false);
    }else if args.reple {
        start_repl(true);
    } else {
        if let Some(file_name) = args.file {
            let parsed = parse_file(&file_name)?;
            print_program(parsed);
        } else  if let Some(file_name) = args.filee {
            let parsed = parse_file(&file_name)?;
            let env = Environment::new(None);
            let interpreter = Interpreter::new(env);
            println!("{:?}", interpreter.evaluate_program(&parsed).unwrap());
        }
    }

    Ok(())
}

fn parse_file(file_name: &str) -> Result<Program, std::io::Error> {
    let file_path = Path::new("programs").join(file_name);
    println!("{}", file_path.to_str().unwrap());
    let program = fs::read_to_string(file_path)?;

    let lexer = Lexer::new(program.to_string());

    let mut parser = parser::Parser::new(lexer);

    // let mut token = lexer.next_token();
    Ok(parser.parse_program().unwrap())
}

fn print_program(program: Program) {
    for statement in &program.statements {
        println!("{}", statement.dbg());
    }

    println!("{program:#?}");
}

fn start_repl(eval: bool) {
    let monkey_face = r#"
    .--.  .-"     "-.  .--.
    / .. \/  .-. .-.  \/ .. \
   | |  '|  /   Y   \  |'  | |      .-"-.            .-"-.            .-"-.   
   | \   \  \ 0 | 0 /  /   / |    _/_-.-_\_        _/.-.-.\_        _/.-.-.\_ 
    \ '- ,\.-"""""""-./, -' /    / __} {__ \      /|( o o )|\      ( ( o o ) )
     ''-' /_   ^ ^   _\ '-''    / //  "  \\ \    | //  "  \\ |      |/  "  \| 
         |  \._   _./  |       / / \'---'/ \ \  / / \'---'/ \ \      \'/^\'/  
         \   \ '~' /   /       \ \_/`"""`\_/ /  \ \_/`"""`\_/ /      /`\ /`\  
          '._ '-=-' _.'         \           /    \           /      /  /|\  \ 
             '-----'
"#;

    println!("{monkey_face}");
    let env = Environment::new(None);
    let interpreter = Interpreter::new(env);

    loop {
        print!("->");

        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim() {
            "E" => break,
            _ => {
                let lexer = Lexer::new(input.to_string());

                // loop {
                //     let token = lexer.next_token();
                //     println!("{:?}", token);
                //     if token.typ == TokenType::Eof { break }
                // }

                let mut parser = parser::Parser::new(lexer);
        
                match parser.parse_program() {
                    Ok(program) => {
                        for statement in &program.statements {
                            println!("{}", statement.dbg());
                        }

                        if eval {
                            println!("******* EVAL *******");
                            println!("{:?}", interpreter.evaluate_program(&program));
                            println!("********************");
                        }
            
                        // println!("{program:#?}")
                    },
                    Err(err) => println!("{err:?}")
                }
            }
        }
    }
}
