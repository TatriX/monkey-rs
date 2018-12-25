use crate::token::Token;
use log::debug;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer = Self {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: 0 as char,
        };
        lexer.read_char();
        lexer
    }
}

fn into_ident_or_keyword(ident: String) -> Token {
    match ident.as_str() {
        "fn" => Token::Function,
        "let" => Token::Let,
        _ => Token::Ident(ident),
    }
}

impl Lexer {
    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }

    fn read_char(&mut self) {
        self.ch = self
            .input
            .get(self.read_position)
            .cloned()
            .unwrap_or(0 as char);
        debug!("Read {:?} from {}", self.ch, self.position);
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> String {
        let start_pos = self.position;
        let mut ident = String::new();

        while self.ch.is_alphabetic() {
            ident.push(self.ch);
            self.read_char();
        }

        debug!(
            "Read ident {} from {} to {}",
            &ident,
            start_pos,
            self.position - 1,
        );

        ident
    }

    fn read_number(&mut self) -> usize {
        let start_pos = self.read_position;
        let mut buf = String::new();

        while self.ch.is_digit(10) {
            buf.push(self.ch);
            self.read_char();
        }

        let number = buf
            .parse::<usize>()
            .expect(&format!("Cannot parse number '{}'", &buf));

        debug!(
            "Read number {} from {} to {}",
            number, start_pos, self.position
        );

        number
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        let token = match self.ch {
            ch if ch == 0 as char => return None,
            '=' => Token::Assign,
            '+' => Token::Plus,
            ';' => Token::Semicolon,
            '(' => Token::Lparen,
            ')' => Token::Rparen,
            ',' => Token::Comma,
            '{' => Token::Lbrace,
            '}' => Token::Rbrace,
            ch if ch.is_alphabetic() => return Some(into_ident_or_keyword(self.read_identifier())),
            ch if ch.is_digit(10) => return Some(Token::Int(self.read_number())),
            ch => Token::Illegal(ch),
        };

        self.read_char();

        debug!("Read token {:?}", &token);

        Some(token)
    }
}
