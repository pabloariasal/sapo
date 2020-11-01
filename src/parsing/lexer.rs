use super::Token;
use std::collections::HashMap;

const EOF: char = '\u{0}';

pub struct Lexer {
    input: String,
    position: usize,
    next_position: usize,
    current_char: char,
    keywords: HashMap<&'static str, Token>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let first = input.chars().next().unwrap_or(EOF);
        Lexer {
            input,
            position: 0,
            next_position: 1,
            current_char: first,
            keywords: initialize_keywords(),
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.current_char {
            EOF => Token::EOF,
            'A'..='Z' | 'a'..='z' | '_' => self.read_identifier(),
            '0'..='9' => self.read_number(),
            '"' => {
                self.read_char();
                self.read_string()
            }
            '-' => Token::Minus,
            '=' => Token::Assign,
            ';' => Token::Semicolon,
            _ => Token::InvalidToken(self.current_char.to_string()),
        };
        self.read_char();
        token
    }

    fn read_string(&mut self) -> Token {
        let pos = self.position;
        self.advance_while(|c| c != '"');
        let identifier = &self.input[pos..self.position];
        Token::StringLiteral(identifier.to_string())
    }

    fn read_number(&mut self) -> Token {
        let pos = self.position;
        self.advance_while(|c| c.is_digit(10));
        let identifier = &self.input[pos..self.position];
        Token::IntegerLiteral(identifier.to_string())
    }

    fn read_identifier(&mut self) -> Token {
        let pos = self.position;
        self.advance_while(|c| c.is_alphanumeric() || c == '_');
        let identifier = &self.input[pos..self.position];
        match self.keywords.get(identifier) {
            Some(token) => token.clone(),
            None => Token::Identifier(identifier.to_string()),
        }
    }

    fn skip_whitespace(&mut self) {
        self.advance_while(|c| c.is_whitespace())
    }

    fn advance_while<P>(&mut self, predicate: P)
    where
        P: Fn(char) -> bool,
    {
        while predicate(self.current_char) {
            self.read_char();
        }
    }

    fn read_char(&mut self) {
        if let Some(c) = self.input.chars().nth(self.next_position) {
            self.current_char = c;
        } else {
            self.current_char = EOF;
        }
        self.position = self.next_position;
        self.next_position += 1;
    }
}

fn initialize_keywords() -> HashMap<&'static str, Token> {
    let mut keywords = HashMap::new();
    keywords.insert("if", Token::If);
    keywords
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexing_empty_string() {
        let mut l = Lexer::new(String::from(""));
        assert_eq!(l.next_token(), Token::EOF);
    }

    #[test]
    fn lex_integers() {
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
        let mut l = Lexer::new(String::from("\"bla bla bla\"  "));
        assert_eq!(
            l.next_token(),
            Token::StringLiteral("bla bla bla".to_string())
        );
        assert_eq!(l.next_token(), Token::EOF);
    }

    #[test]
    fn lex_if() {
        let mut l = Lexer::new(String::from("if"));
        assert_eq!(l.next_token(), Token::If);
        assert_eq!(l.next_token(), Token::EOF);
    }

    #[ignore]
    #[test]
    fn lex_equals_assign() {
        let mut l = Lexer::new(String::from("= =="));
        assert_eq!(l.next_token(), Token::Assign);
        assert_eq!(l.next_token(), Token::Equals);
        assert_eq!(l.next_token(), Token::EOF);
    }

    #[ignore]
    #[test]
    fn lexing_combined() {
        let input = r#"
           x = -4;

        yolo = 56789iii
        z42     = "mono is cool"
        #if

        "#;

        let expected_tokens = [
            Token::Identifier(String::from("x")),
            Token::Assign,
            Token::Minus,
            Token::IntegerLiteral(String::from("4")),
            Token::Semicolon,
            Token::Identifier(String::from("yolo")),
            Token::Assign,
            Token::IntegerLiteral(String::from("56789")),
            Token::StringLiteral(String::from("iii")),
            Token::Identifier(String::from("z42")),
            Token::Assign,
            Token::StringLiteral(String::from("mono is cool")),
            Token::InvalidToken(String::from("#")),
            Token::If,
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
