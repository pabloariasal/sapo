use std::io;
use std::io::Write;

fn main() {
    println!("Sapo Programming Language 🐸");
    loop {
        print!("🐸> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "exit" => break,
            _ => match sapo::parse(input) {
                Ok(ast) => match sapo::evaluate(&*ast) {
                    Ok(result) => {
                        // println!("{}", sapo::ast_printer::print_ast(ast));
                        println!("{}", result);
                    }
                    Err(error) => println!("{}", error),
                },
                Err(error) => println!("{}", error),
            },
        };
    }
}
