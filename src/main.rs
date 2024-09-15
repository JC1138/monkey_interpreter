use clap::Parser;
use std::fs;
use std::path::Path;

use std::io::{self, Write};

use monkey_interpreter::{lexer::Lexer, parser};

/// A simple program to read a file from the programs directory
#[derive(Parser)]
struct Args {
    /// The file name to read (located in /programs directory)
    #[arg(long)]
    file: Option<String>,

    #[arg(long, action = clap::ArgAction::SetTrue)]
    repl: bool,
}

fn main() -> Result<(), std::io::Error> {
    
    let args = Args::parse();

    if args.repl {
        start_repl();
    } else {
        if let Some(file_name) = args.file {
            parse_file(&file_name)?;
        };
    }

    Ok(())
}

fn parse_file(file_name: &str) -> Result<(), std::io::Error> {
    let file_path = Path::new("programs").join(file_name);
    println!("{}", file_path.to_str().unwrap());
    // Read the file contents
    let program = fs::read_to_string(file_path)?;

    // Print the file contents
    // println!("File contents:\n{}", contents);

    let lexer = Lexer::new(program.to_string());

    let mut parser = parser::Parser::new(lexer);

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


    return Ok(())
}

fn start_repl() {
    let monkey_face = r#"
    .--.  .-"     "-.  .--.
    / .. \/  .-. .-.  \/ .. \
   | |  '|  /   Y   \  |'  | |
   | \   \  \ 0 | 0 /  /   / |
    \ '- ,\.-"""""""-./, -' /
     ''-' /_   ^ ^   _\ '-''
         |  \._   _./  |
         \   \ '~' /   /
          '._ '-=-' _.'
             '-----'
"#;

    println!("{monkey_face}");

    loop {
        print!("->");

        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim() {
            "E" => break,
            _ => {
                let lexer = Lexer::new(input.to_string());
                let mut parser = parser::Parser::new(lexer);
        
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
    }
}
