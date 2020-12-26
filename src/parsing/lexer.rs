use crate::token::Token;
use crate::token::TokenType;
use std::collections::HashMap;

const EOF: char = '\u{0}';

struct Keyword {
    token_type: TokenType,
    lexeme: &'static str,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    next_position: usize,
    current_char: char,
    keywords: HashMap<String, Keyword>,
    current_line: i32,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            input: input.chars().collect::<Vec<_>>(),
            position: 0,
            next_position: 0,
            current_char: EOF,
            keywords: initialize_keywords(),
            current_line: 1,
        }
    }

    fn next_token(&mut self) -> Token {
        self.advance();
        //move to first non-whitespace character
        self.advance_until(|c| !c.is_whitespace());
        match self.current_char {
            EOF => Token::new(TokenType::EOF, "EOF".to_string(), self.current_line),
            c if is_digit(c) => self.read_number(),
            c if is_alpha(c) => self.read_identifier(),
            '"' => self.read_string(),
            '-' => Token::new(
                TokenType::Minus,
                self.current_char.to_string(),
                self.current_line,
            ),
            '+' => Token::new(
                TokenType::Plus,
                self.current_char.to_string(),
                self.current_line,
            ),
            '*' => Token::new(
                TokenType::Star,
                self.current_char.to_string(),
                self.current_line,
            ),
            '/' => Token::new(
                TokenType::Slash,
                self.current_char.to_string(),
                self.current_line,
            ),
            '(' => Token::new(
                TokenType::LeftParen,
                self.current_char.to_string(),
                self.current_line,
            ),
            ')' => Token::new(
                TokenType::RightParen,
                self.current_char.to_string(),
                self.current_line,
            ),
            '{' => Token::new(
                TokenType::LeftBrace,
                self.current_char.to_string(),
                self.current_line,
            ),
            '}' => Token::new(
                TokenType::RightBrace,
                self.current_char.to_string(),
                self.current_line,
            ),
            '=' => {
                if self.matches('=') {
                    Token::new(TokenType::Equals, "==".to_string(), self.current_line)
                } else {
                    Token::new(
                        TokenType::Assignment,
                        self.current_char.to_string(),
                        self.current_line,
                    )
                }
            }
            '!' => {
                if self.matches('=') {
                    Token::new(TokenType::BangEquals, "!=".to_string(), self.current_line)
                } else {
                    Token::new(
                        TokenType::Bang,
                        self.current_char.to_string(),
                        self.current_line,
                    )
                }
            }
            '<' => {
                if self.matches('=') {
                    Token::new(
                        TokenType::SmallerEquals,
                        "<=".to_string(),
                        self.current_line,
                    )
                } else {
                    Token::new(
                        TokenType::Smaller,
                        self.current_char.to_string(),
                        self.current_line,
                    )
                }
            }
            '>' => {
                if self.matches('=') {
                    Token::new(
                        TokenType::GreaterEquals,
                        ">=".to_string(),
                        self.current_line,
                    )
                } else {
                    Token::new(
                        TokenType::Greater,
                        self.current_char.to_string(),
                        self.current_line,
                    )
                }
            }
            ';' => Token::new(
                TokenType::Semicolon,
                self.current_char.to_string(),
                self.current_line,
            ),
            _ => Token::new(
                TokenType::InvalidToken,
                self.current_char.to_string(),
                self.current_line,
            ),
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
            self.current_line,
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
            self.current_line,
        )
    }

    fn read_identifier(&mut self) -> Token {
        let start = self.position;
        self.advance_while(is_alpha);
        let identifier = self.extract_substring(start, self.position + 1);
        match self.keywords.get(&identifier) {
            Some(Keyword { token_type, lexeme }) => {
                Token::new(token_type.clone(), lexeme.to_string(), self.current_line)
            }
            None => Token::new(TokenType::Identifier, identifier, self.current_line),
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

        if self.current_char == '\n' {
            self.current_line += 1;
        }
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

fn initialize_keywords() -> HashMap<String, Keyword> {
    let mut keywords = HashMap::new();
    keywords.insert(
        "if".to_string(),
        Keyword {
            token_type: TokenType::If,
            lexeme: "if",
        },
    );
    keywords.insert(
        "true".to_string(),
        Keyword {
            token_type: TokenType::BooleanLiteral,
            lexeme: "true",
        },
    );
    keywords.insert(
        "false".to_string(),
        Keyword {
            token_type: TokenType::BooleanLiteral,
            lexeme: "false",
        },
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
        assert_eq!(
            *p.peek().unwrap(),
            Token::new(TokenType::BooleanLiteral, "true".to_string(), 1)
        );
        assert_eq!(
            p.next().unwrap(),
            Token::new(TokenType::BooleanLiteral, "true".to_string(), 1)
        );
        assert_eq!(
            *p.peek().unwrap(),
            Token::new(TokenType::BooleanLiteral, "false".to_string(), 1)
        );
        assert_eq!(
            p.next().unwrap(),
            Token::new(TokenType::BooleanLiteral, "false".to_string(), 1)
        );
        assert_eq!(p.next(), None)
    }

    #[test]
    fn lex_boolean_expressions() {
        let mut l = Lexer::new(String::from("true false !true"));
        assert_eq!(
            l.next().unwrap(),
            Token::new(TokenType::BooleanLiteral, "true".to_string(), 1)
        );
        assert_eq!(
            l.next().unwrap(),
            Token::new(TokenType::BooleanLiteral, "false".to_string(), 1)
        );
        assert_eq!(
            l.next().unwrap(),
            Token::new(TokenType::Bang, "!".to_string(), 1)
        );
        assert_eq!(
            l.next().unwrap(),
            Token::new(TokenType::BooleanLiteral, "true".to_string(), 1)
        );
        assert_eq!(l.next(), None)
    }

    #[test]
    fn lex_comparison_operators() {
        let mut l = Lexer::new(String::from("= == != <= >= <>".to_string()));
        assert_eq!(
            l.next().unwrap(),
            Token::new(TokenType::Assignment, "=".to_string(), 1)
        );
        assert_eq!(
            l.next().unwrap(),
            Token::new(TokenType::Equals, "==".to_string(), 1)
        );
        assert_eq!(
            l.next().unwrap(),
            Token::new(TokenType::BangEquals, "!=".to_string(), 1)
        );
        assert_eq!(
            l.next().unwrap(),
            Token::new(TokenType::SmallerEquals, "<=".to_string(), 1)
        );
        assert_eq!(
            l.next().unwrap(),
            Token::new(TokenType::GreaterEquals, ">=".to_string(), 1)
        );
        assert_eq!(
            l.next().unwrap(),
            Token::new(TokenType::Smaller, "<".to_string(), 1)
        );
        assert_eq!(
            l.next().unwrap(),
            Token::new(TokenType::Greater, ">".to_string(), 1)
        );
        assert_eq!(l.next(), None)
    }

    #[test]
    fn lex_parenthesis() {
        let mut l = Lexer::new(String::from("({}( ))"));
        assert_eq!(
            l.next().unwrap(),
            Token::new(TokenType::LeftParen, "(".to_string(), 1)
        );
        assert_eq!(
            l.next().unwrap(),
            Token::new(TokenType::LeftBrace, "{".to_string(), 1)
        );
        assert_eq!(
            l.next().unwrap(),
            Token::new(TokenType::RightBrace, "}".to_string(), 1)
        );
        assert_eq!(
            l.next().unwrap(),
            Token::new(TokenType::LeftParen, "(".to_string(), 1)
        );
        assert_eq!(
            l.next().unwrap(),
            Token::new(TokenType::RightParen, ")".to_string(), 1)
        );
        assert_eq!(
            l.next().unwrap(),
            Token::new(TokenType::RightParen, ")".to_string(), 1)
        );
        assert_eq!(l.next(), None)
    }

    #[test]
    fn lex_arithmetic_operators() {
        let mut l = Lexer::new(String::from(" + - */"));
        assert_eq!(
            l.next().unwrap(),
            Token::new(TokenType::Plus, "+".to_string(), 1)
        );
        assert_eq!(
            l.next().unwrap(),
            Token::new(TokenType::Minus, "-".to_string(), 1)
        );
        assert_eq!(
            l.next().unwrap(),
            Token::new(TokenType::Star, "*".to_string(), 1)
        );
        assert_eq!(
            l.next().unwrap(),
            Token::new(TokenType::Slash, "/".to_string(), 1)
        );
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
        assert_eq!(
            l.next().unwrap(),
            Token::new(TokenType::IntegerLiteral, "5".to_string(), 1)
        );
        assert_eq!(
            l.next().unwrap(),
            Token::new(TokenType::IntegerLiteral, "88989".to_string(), 1)
        );
        assert_eq!(
            l.next().unwrap(),
            Token::new(TokenType::Minus, "-".to_string(), 1)
        );
        assert_eq!(
            l.next().unwrap(),
            Token::new(TokenType::IntegerLiteral, "2928".to_string(), 1)
        );
        assert_eq!(l.next(), None)
    }

    #[test]
    fn lex_identifiers() {
        let input = "_x x_x_x78 Yh0A99";
        let mut l = Lexer::new(String::from(input.to_string()));
        assert_eq!(
            l.next_token(),
            Token::new(TokenType::Identifier, "_x".to_string(), 1)
        );
        assert_eq!(
            l.next_token(),
            Token::new(TokenType::Identifier, "x_x_x78".to_string(), 1)
        );
        assert_eq!(
            l.next_token(),
            Token::new(TokenType::Identifier, "Yh0A99".to_string(), 1)
        );
        assert_eq!(l.next(), None)
    }

    #[test]
    fn lex_invalid_tokens() {
        let mut l = Lexer::new(String::from("#"));
        assert_eq!(
            l.next_token(),
            Token::new(TokenType::InvalidToken, "#".to_string(), 1)
        );
        assert_eq!(l.next(), None)
    }

    #[test]
    fn lex_strings() {
        let mut l = Lexer::new(String::from("\"bla bla bla\"  "));
        assert_eq!(
            l.next_token(),
            Token::new(TokenType::StringLiteral, "bla bla bla".to_string(), 1)
        );
        assert_eq!(l.next(), None)
    }

    #[test]
    fn lex_if() {
        let mut l = Lexer::new(String::from("if"));
        assert_eq!(
            l.next_token(),
            Token::new(TokenType::If, "if".to_string(), 1)
        );
        assert_eq!(l.next(), None)
    }

    #[test]
    fn lex_semicolon() {
        let mut l = Lexer::new(String::from("47;"));
        assert_eq!(
            l.next_token(),
            Token::new(TokenType::IntegerLiteral, "47".to_string(), 1)
        );
        assert_eq!(
            l.next_token(),
            Token::new(TokenType::Semicolon, ";".to_string(), 1)
        );
        assert_eq!(l.next(), None)
    }

    #[test]
    fn lex_combined() {
        let input = r#"
           x = -4;

        yolo = 56789"iii"
        z42     = "sapo is cool"
        #if==

        "#;

        let mut l = Lexer::new(input.to_string());

        let expected_tokens = [
            Token::new(TokenType::Identifier, "x".to_string(), 2),
            Token::new(TokenType::Assignment, "=".to_string(), 2),
            Token::new(TokenType::Minus, "-".to_string(), 2),
            Token::new(TokenType::IntegerLiteral, "4".to_string(), 2),
            Token::new(TokenType::Semicolon, ";".to_string(), 2),
            Token::new(TokenType::Identifier, "yolo".to_string(), 4),
            Token::new(TokenType::Assignment, "=".to_string(), 4),
            Token::new(TokenType::IntegerLiteral, "56789".to_string(), 4),
            Token::new(TokenType::StringLiteral, "iii".to_string(), 4),
            Token::new(TokenType::Identifier, "z42".to_string(), 5),
            Token::new(TokenType::Assignment, "=".to_string(), 5),
            Token::new(TokenType::StringLiteral, "sapo is cool".to_string(), 5),
            Token::new(TokenType::InvalidToken, "#".to_string(), 6),
            Token::new(TokenType::If, "if".to_string(), 6),
            Token::new(TokenType::Equals, "==".to_string(), 6),
        ];

        for expected in expected_tokens.iter() {
            let actual = l.next_token();
            assert_eq!(expected, &actual);
        }
        assert_eq!(l.next(), None)
    }
}
