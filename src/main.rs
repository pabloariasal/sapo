use mono::parsing::Lexer;
use mono::parsing::Token;
use std::io;
use std::io::Write;

fn main() {
    loop {
        mono::test_func();
        print!("=> ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "exit" => break,
            _ => {
                let mut lexer = Lexer::new(input);
                loop
                {
                    let token = lexer.next_token();
                    match token
                    {
                        Token::EOF => break,
                        _ => println!("{:?}", token)
                    }
                }
            }
        };
    }
}
