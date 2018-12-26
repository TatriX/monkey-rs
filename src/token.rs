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
        macro_rules! format_ident {
            ($fmt:expr, $($arg:tt)*) => {
                {
                    buf = format!($fmt, $($arg)*);
                    buf.as_str()
                }
            }
        }

        write!(
            f,
            "{}",
            match self {
                Illegal(c) => format_ident!("Illegal({})", c),
                Ident(ident) => format_ident!("Ident({})", ident),
                Int(i) => format_ident!("{}", i.to_string()),

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
