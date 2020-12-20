use super::Token;
use std::collections::HashMap;

const EOF: char = '\u{0}';

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    next_position: usize,
    current_char: char,
    keywords: HashMap<String, Token>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            input: input.chars().collect::<Vec<_>>(),
            position: 0,
            next_position: 0,
            current_char: EOF,
            keywords: initialize_keywords(),
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.advance();
        //move to first non-whitespace character
        self.advance_until(|c| !c.is_whitespace());
        match self.current_char {
            EOF => Token::EOF,
            c if is_digit(c) => self.read_number(),
            c if is_alpha(c) => self.read_identifier(),
            '"' => self.read_string(),
            '-' => Token::Minus,
            '+' => Token::Plus,
            '*' => Token::Dot,
            '/' => Token::Slash,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '=' => {
                if self.matches('=') {
                    Token::Equals
                } else {
                    Token::Assignment
                }
            }
            '!' => {
                if self.matches('=') {
                    Token::BangEquals
                } else {
                    Token::Bang
                }
            }
            '<' => {
                if self.matches('=') {
                    Token::SmallerEquals
                } else {
                    Token::Smaller
                }
            }
            '>' => {
                if self.matches('=') {
                    Token::GreaterEquals
                } else {
                    Token::Greater
                }
            }
            ';' => Token::Semicolon,
            _ => Token::InvalidToken(self.current_char.to_string()),
        }
    }

    fn read_string(&mut self) -> Token {
        // advance opening '"'
        self.advance();
        let start = self.position;
        self.advance_while(|c| c != '"');
        let t = Token::StringLiteral(self.extract_substring(start, self.position + 1));
        // advance closing '"'
        self.advance();
        t
    }

    fn read_number(&mut self) -> Token {
        let start = self.position;
        self.advance_while(is_digit);
        Token::IntegerLiteral(self.extract_substring(start, self.position + 1))
    }

    fn read_identifier(&mut self) -> Token {
        let start = self.position;
        self.advance_while(is_alpha);
        let identifier = self.extract_substring(start, self.position + 1);
        match self.keywords.get(&identifier) {
            Some(token) => token.clone(),
            None => Token::Identifier(identifier),
        }
    }

    /// Advances the lexer until the current char passes the predicate
    /// or until the end of the input
    /// useful for jumping to the next non-whitespace character
    fn advance_until<P>(&mut self, predicate: P)
    where
        P: Fn(char) -> bool,
    {
        while self.current_char != EOF && !predicate(self.current_char) {
            self.advance();
        }
    }

    /// Advances the lexer as long as the current char passes the predicate
    /// or until the end of the input
    /// Useful when reading multi-character tokens like string or numeric literals
    fn advance_while<P>(&mut self, predicate: P)
    where
        P: Fn(char) -> bool,
    {
        while self.current_char != EOF && predicate(self.current_char) && predicate(self.peek()) {
            self.advance();
        }
    }

    fn advance(&mut self) {
        self.current_char = match self.input.get(self.next_position) {
            Some(&c) => c,
            None => EOF,
        };
        self.position = self.next_position;
        self.next_position += 1;
    }

    fn peek(&self) -> char {
        match self.input.get(self.next_position) {
            Some(&c) => c,
            _ => EOF,
        }
    }

    /// Conditional advance.
    /// Advances the lexer if the next character matches expected
    /// Returns true is the lexer was advanced, false otherwise
    fn matches(&mut self, expected: char) -> bool {
        if self.peek() == expected {
            self.advance();
            return true;
        }
        false
    }

    fn extract_substring(&self, from: usize, to: usize) -> String {
        (&self.input[from..to]).iter().collect()
    }
}

fn initialize_keywords() -> HashMap<String, Token> {
    let mut keywords = HashMap::new();
    keywords.insert("if".to_string(), Token::If);
    keywords
}

fn is_alpha(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_comparison_operators() {
        let mut l = Lexer::new(String::from("= == != <= >= <>"));
        assert_eq!(l.next_token(), Token::Assignment);
        assert_eq!(l.next_token(), Token::Equals);
        assert_eq!(l.next_token(), Token::BangEquals);
        assert_eq!(l.next_token(), Token::SmallerEquals);
        assert_eq!(l.next_token(), Token::GreaterEquals);
        assert_eq!(l.next_token(), Token::Smaller);
        assert_eq!(l.next_token(), Token::Greater);
        assert_eq!(l.next_token(), Token::EOF);
    }

    #[test]
    fn lex_parenthesis() {
        let mut l = Lexer::new(String::from("(( ))"));
        assert_eq!(l.next_token(), Token::LeftParen);
        assert_eq!(l.next_token(), Token::LeftParen);
        assert_eq!(l.next_token(), Token::RightParen);
        assert_eq!(l.next_token(), Token::RightParen);
        assert_eq!(l.next_token(), Token::EOF);
    }

    #[test]
    fn lex_arithmetic_operators() {
        let mut l = Lexer::new(String::from(" + - */"));
        assert_eq!(l.next_token(), Token::Plus);
        assert_eq!(l.next_token(), Token::Minus);
        assert_eq!(l.next_token(), Token::Dot);
        assert_eq!(l.next_token(), Token::Slash);
    }

    #[test]
    fn lex_empty_string() {
        let mut l = Lexer::new(String::from(""));
        assert_eq!(l.next_token(), Token::EOF);
    }

    #[test]
    fn lex_whitespace() {
        let mut l = Lexer::new(String::from("\r \t \n   "));
        assert_eq!(l.next_token(), Token::EOF);
    }

    #[test]
    fn lex_integral_literals() {
        let input = "5 88989 -2928";
        let mut l = Lexer::new(String::from(input));
        assert_eq!(l.next_token(), Token::IntegerLiteral("5".to_string()));
        assert_eq!(l.next_token(), Token::IntegerLiteral("88989".to_string()));
        assert_eq!(l.next_token(), Token::Minus);
        assert_eq!(l.next_token(), Token::IntegerLiteral("2928".to_string()));
        assert_eq!(l.next_token(), Token::EOF);
    }

    #[test]
    fn lex_identifiers() {
        let input = "_x x_x_x78 Yh0A99";
        let mut l = Lexer::new(String::from(input.to_string()));
        assert_eq!(l.next_token(), Token::Identifier("_x".to_string()));
        assert_eq!(l.next_token(), Token::Identifier("x_x_x78".to_string()));
        assert_eq!(l.next_token(), Token::Identifier("Yh0A99".to_string()));
        assert_eq!(l.next_token(), Token::EOF);
    }

    #[test]
    fn lex_invalid_tokens() {
        let mut l = Lexer::new(String::from("#"));
        assert_eq!(l.next_token(), Token::InvalidToken("#".to_string()));
        assert_eq!(l.next_token(), Token::EOF);
    }

    #[test]
    fn lex_strings() {
        let mut l = Lexer::new(String::from("\"bla \n bla bla\"  "));
        assert_eq!(
            l.next_token(),
            Token::StringLiteral("bla \n bla bla".to_string())
        );
        assert_eq!(l.next_token(), Token::EOF);
    }

    #[test]
    fn lex_if() {
        let mut l = Lexer::new(String::from("if"));
        assert_eq!(l.next_token(), Token::If);
        assert_eq!(l.next_token(), Token::EOF);
    }

    #[test]
    fn lex_semicolon() {
        let mut l = Lexer::new(String::from("47;"));
        assert_eq!(l.next_token(), Token::IntegerLiteral("47".to_string()));
        assert_eq!(l.next_token(), Token::Semicolon);
        assert_eq!(l.next_token(), Token::EOF);
    }

    #[test]
    fn lex_combined() {
        let input = r#"
           x = -4;

        yolo = 56789"iii"
        z42     = "mono is cool"
        #if==

        "#;

        let expected_tokens = [
            Token::Identifier(String::from("x")),
            Token::Assignment,
            Token::Minus,
            Token::IntegerLiteral(String::from("4")),
            Token::Semicolon,
            Token::Identifier(String::from("yolo")),
            Token::Assignment,
            Token::IntegerLiteral(String::from("56789")),
            Token::StringLiteral(String::from("iii")),
            Token::Identifier(String::from("z42")),
            Token::Assignment,
            Token::StringLiteral(String::from("mono is cool")),
            Token::InvalidToken(String::from("#")),
            Token::If,
            Token::Equals,
            Token::EOF,
            Token::EOF,
            Token::EOF,
        ];
        let mut l = Lexer::new(input.to_string());

        for expected in expected_tokens.iter() {
            let actual = l.next_token();
            assert_eq!(
                expected, &actual,
                "Expected equality of {:?} and {:?}",
                expected, &actual
            );
        }
    }
}
