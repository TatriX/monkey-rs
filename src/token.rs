use std::fmt::{self, Debug};

#[derive(PartialEq)]
pub enum Token {
    Illegal(char),
    // For Eof, lexer.next() will just return None
    Ident(String),
    Int(usize),

    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    Lt,
    Gt,
    Eq,
    NotEq,

    Comma,
    Semicolon,

    Lparen,
    Rparen,
    Lbrace,
    Rbrace,

    Function,
    Let,

    If,
    Else,
    Return,

    True,
    False,
}

impl Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Token::*;

        let buf;
        write!(
            f,
            "{}",
            match self {
                Illegal(c) => {
                    buf = format!("Illegal({})", c);
                    buf.as_str()
                }
                Ident(ident) => ident,
                Int(i) => {
                    buf = i.to_string();
                    buf.as_str()
                }

                Assign => "=",
                Plus => "+",
                Minus => "-",
                Bang => "!",
                Asterisk => "*",
                Slash => "/",

                Lt => "<",
                Gt => ">",
                Eq => "==",
                NotEq => "!=",

                Comma => ",",
                Semicolon => ";",

                Lparen => "(",
                Rparen => ")",
                Lbrace => "{",
                Rbrace => "}",

                Function => "fn",
                Let => "let",

                If => "if",
                Else => "else",
                Return => "return",

                True => "true",
                False => "false",
            }
        )
    }
}
