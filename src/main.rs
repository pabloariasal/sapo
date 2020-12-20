use std::io;
use std::io::Write;
use sapo::parsing::Lexer;
use sapo::parsing::TokenType;

fn main() {
    println!("Sapo Programming Language");
    loop {
        print!("=> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "exit" => break,
            _ => {
                let mut lexer = Lexer::new(input);
                loop {
                    let token = lexer.next_token();
                    println!("{:?}", token);
                    if token.token_type == TokenType::EOF {
                        break;
                    }
                }
            }
        };
    }
}
