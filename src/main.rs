use std::io;
use std::io::Write;

fn main() {
    println!("Sapo Programming Language");
    loop {
        print!("=> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "exit" => break,
            _ => match sapo::parse(input) {
                Ok(ast) => println!("{}", sapo::ast_printer::print_ast(ast)),
                Err(error) => println!("{}", error),
            },
        };
    }
}
