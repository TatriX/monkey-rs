#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal(char),
    // For Eof, lexer.next() will just return None
    Ident(String),
    Int(usize),

    Assign,
    Plus,

    Comma,
    Semicolon,

    Lparen,
    Rparen,
    Lbrace,
    Rbrace,

    Function,
    Let,
}
