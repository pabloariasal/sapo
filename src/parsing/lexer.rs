use crate::token::Token;
use crate::token::TokenType;
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

    fn next_token(&mut self) -> Token {
        self.advance();
        //move to first non-whitespace character
        self.advance_until(|c| !c.is_whitespace());
        match self.current_char {
            EOF => token(TokenType::EOF, "EOF"),
            c if is_digit(c) => self.read_number(),
            c if is_alpha(c) => self.read_identifier(),
            '"' => self.read_string(),
            '-' => token(TokenType::Minus, &self.current_char.to_string()),
            '+' => token(TokenType::Plus, &self.current_char.to_string()),
            '*' => token(TokenType::Star, &self.current_char.to_string()),
            '/' => token(TokenType::Slash, &self.current_char.to_string()),
            '(' => token(TokenType::LeftParen, &self.current_char.to_string()),
            ')' => token(TokenType::RightParen, &self.current_char.to_string()),
            '{' => token(TokenType::LeftBrace, &self.current_char.to_string()),
            '}' => token(TokenType::RightBrace, &self.current_char.to_string()),
            '=' => {
                if self.matches('=') {
                    token(TokenType::Equals, "==")
                } else {
                    token(TokenType::Assignment, &self.current_char.to_string())
                }
            }
            '!' => {
                if self.matches('=') {
                    token(TokenType::BangEquals, "!=")
                } else {
                    token(TokenType::Bang, &self.current_char.to_string())
                }
            }
            '<' => {
                if self.matches('=') {
                    token(TokenType::SmallerEquals, "<=")
                } else {
                    token(TokenType::Smaller, &self.current_char.to_string())
                }
            }
            '>' => {
                if self.matches('=') {
                    token(TokenType::GreaterEquals, ">=")
                } else {
                    token(TokenType::Greater, &self.current_char.to_string())
                }
            }
            ';' => token(TokenType::Semicolon, &self.current_char.to_string()),
            _ => token(TokenType::InvalidToken, &self.current_char.to_string()),
        }
    }

    fn read_string(&mut self) -> Token {
        // advance opening '"'
        self.advance();
        let start = self.position;
        self.advance_while(|c| c != '"');
        let t = Token::new(
            TokenType::StringLiteral,
            self.extract_substring(start, self.position + 1),
        );
        // advance closing '"'
        self.advance();
        t
    }

    fn read_number(&mut self) -> Token {
        let start = self.position;
        self.advance_while(is_digit);
        Token::new(
            TokenType::IntegerLiteral,
            self.extract_substring(start, self.position + 1),
        )
    }

    fn read_identifier(&mut self) -> Token {
        let start = self.position;
        self.advance_while(is_alpha);
        let identifier = self.extract_substring(start, self.position + 1);
        match self.keywords.get(&identifier) {
            Some(token) => token.clone(),
            None => Token::new(TokenType::Identifier, identifier),
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

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let next_token = self.next_token();
        if next_token.token_type == TokenType::EOF {
            None
        } else {
            Some(next_token)
        }
    }
}

fn token(token_type: TokenType, lexeme: &str) -> Token {
    Token::new(token_type, lexeme.to_string())
}

fn initialize_keywords() -> HashMap<String, Token> {
    let mut keywords = HashMap::new();
    keywords.insert("if".to_string(), token(TokenType::If, "if"));
    keywords.insert("true".to_string(), token(TokenType::BooleanLiteral, "true"));
    keywords.insert(
        "false".to_string(),
        token(TokenType::BooleanLiteral, "false"),
    );
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
    fn peek_token() {
        let l = Lexer::new(String::from("true false"));
        let mut p = l.into_iter().peekable();
        assert_eq!(*p.peek().unwrap(), token(TokenType::BooleanLiteral, "true"));
        assert_eq!(p.next().unwrap(), token(TokenType::BooleanLiteral, "true"));
        assert_eq!(*p.peek().unwrap(), token(TokenType::BooleanLiteral, "false"));
        assert_eq!(p.next().unwrap(), token(TokenType::BooleanLiteral, "false"));
        assert_eq!(p.next(), None)
    }

    #[test]
    fn lex_boolean_expressions() {
        let mut l = Lexer::new(String::from("true false !true"));
        assert_eq!(l.next().unwrap(), token(TokenType::BooleanLiteral, "true"));
        assert_eq!(l.next().unwrap(), token(TokenType::BooleanLiteral, "false"));
        assert_eq!(l.next().unwrap(), token(TokenType::Bang, "!"));
        assert_eq!(l.next().unwrap(), token(TokenType::BooleanLiteral, "true"));
        assert_eq!(l.next(), None)
    }

    #[test]
    fn lex_comparison_operators() {
        let mut l = Lexer::new(String::from("= == != <= >= <>"));
        assert_eq!(l.next().unwrap(), token(TokenType::Assignment, "="));
        assert_eq!(l.next().unwrap(), token(TokenType::Equals, "=="));
        assert_eq!(l.next().unwrap(), token(TokenType::BangEquals, "!="));
        assert_eq!(l.next().unwrap(), token(TokenType::SmallerEquals, "<="));
        assert_eq!(l.next().unwrap(), token(TokenType::GreaterEquals, ">="));
        assert_eq!(l.next().unwrap(), token(TokenType::Smaller, "<"));
        assert_eq!(l.next().unwrap(), token(TokenType::Greater, ">"));
        assert_eq!(l.next(), None)
    }

    #[test]
    fn lex_parenthesis() {
        let mut l = Lexer::new(String::from("({}( ))"));
        assert_eq!(l.next().unwrap(), token(TokenType::LeftParen, "("));
        assert_eq!(l.next().unwrap(), token(TokenType::LeftBrace, "{"));
        assert_eq!(l.next().unwrap(), token(TokenType::RightBrace, "}"));
        assert_eq!(l.next().unwrap(), token(TokenType::LeftParen, "("));
        assert_eq!(l.next().unwrap(), token(TokenType::RightParen, ")"));
        assert_eq!(l.next().unwrap(), token(TokenType::RightParen, ")"));
        assert_eq!(l.next(), None)
    }

    #[test]
    fn lex_arithmetic_operators() {
        let mut l = Lexer::new(String::from(" + - */"));
        assert_eq!(l.next().unwrap(), token(TokenType::Plus, "+"));
        assert_eq!(l.next().unwrap(), token(TokenType::Minus, "-"));
        assert_eq!(l.next().unwrap(), token(TokenType::Star, "*"));
        assert_eq!(l.next().unwrap(), token(TokenType::Slash, "/"));
        assert_eq!(l.next(), None)
    }

    #[test]
    fn lex_empty_string() {
        let mut l = Lexer::new(String::from(""));
        assert_eq!(l.next(), None)
    }

    #[test]
    fn lex_whitespace() {
        let mut l = Lexer::new(String::from("\r \t \n   "));
        assert_eq!(l.next(), None)
    }

    #[test]
    fn lex_integral_literals() {
        let input = "5 88989 -2928";
        let mut l = Lexer::new(String::from(input));
        assert_eq!(l.next().unwrap(), token(TokenType::IntegerLiteral, "5"));
        assert_eq!(l.next().unwrap(), token(TokenType::IntegerLiteral, "88989"));
        assert_eq!(l.next().unwrap(), token(TokenType::Minus, "-"));
        assert_eq!(l.next().unwrap(), token(TokenType::IntegerLiteral, "2928"));
        assert_eq!(l.next(), None)
    }

    #[test]
    fn lex_identifiers() {
        let input = "_x x_x_x78 Yh0A99";
        let mut l = Lexer::new(String::from(input.to_string()));
        assert_eq!(l.next_token(), token(TokenType::Identifier, "_x"));
        assert_eq!(l.next_token(), token(TokenType::Identifier, "x_x_x78"));
        assert_eq!(l.next_token(), token(TokenType::Identifier, "Yh0A99"));
        assert_eq!(l.next(), None)
    }

    #[test]
    fn lex_invalid_tokens() {
        let mut l = Lexer::new(String::from("#"));
        assert_eq!(l.next_token(), token(TokenType::InvalidToken, "#"));
        assert_eq!(l.next(), None)
    }

    #[test]
    fn lex_strings() {
        let mut l = Lexer::new(String::from("\"bla \n bla bla\"  "));
        assert_eq!(
            l.next_token(),
            token(TokenType::StringLiteral, "bla \n bla bla")
        );
        assert_eq!(l.next(), None)
    }

    #[test]
    fn lex_if() {
        let mut l = Lexer::new(String::from("if"));
        assert_eq!(l.next_token(), token(TokenType::If, "if"));
        assert_eq!(l.next(), None)
    }

    #[test]
    fn lex_semicolon() {
        let mut l = Lexer::new(String::from("47;"));
        assert_eq!(l.next_token(), token(TokenType::IntegerLiteral, "47"));
        assert_eq!(l.next_token(), token(TokenType::Semicolon, ";"));
        assert_eq!(l.next(), None)
    }

    #[test]
    fn lex_combined() {
        let input = r#"
           x = -4;

        yolo = 56789"iii"
        z42     = "mono is cool"
        #if==

        "#;

        let mut l = Lexer::new(input.to_string());

        let expected_tokens = [
            token(TokenType::Identifier, "x"),
            token(TokenType::Assignment, "="),
            token(TokenType::Minus, "-"),
            token(TokenType::IntegerLiteral, "4"),
            token(TokenType::Semicolon, ";"),
            token(TokenType::Identifier, "yolo"),
            token(TokenType::Assignment, "="),
            token(TokenType::IntegerLiteral, "56789"),
            token(TokenType::StringLiteral, "iii"),
            token(TokenType::Identifier, "z42"),
            token(TokenType::Assignment, "="),
            token(TokenType::StringLiteral, "mono is cool"),
            token(TokenType::InvalidToken, "#"),
            token(TokenType::If, "if"),
            token(TokenType::Equals, "=="),
        ];

        for expected in expected_tokens.iter() {
            let actual = l.next_token();
            assert_eq!(
                expected, &actual,
                "Expected equality of {:?} and {:?}",
                expected, &actual
            );
        }
        assert_eq!(l.next(), None)
    }
}
