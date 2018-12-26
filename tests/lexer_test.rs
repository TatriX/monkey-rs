use env_logger;
use monkey::lexer::Lexer;

#[test]
fn test_next_token() {
    use monkey::token::Token::*;

    let _ = env_logger::try_init();

    let input = "
let five = 5;
let ten = 10;

let add = fn(x, y) {
  x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
	return true;
} else {
	return false;
}

10 == 10;
10 != 9;
";
    let expected = vec![
        Let,
        Ident("five".into()),
        Assign,
        Int(5),
        Semicolon,
        Let,
        Ident("ten".into()),
        Assign,
        Int(10),
        Semicolon,
        Let,
        Ident("add".into()),
        Assign,
        Function,
        Lparen,
        Ident("x".into()),
        Comma,
        Ident("y".into()),
        Rparen,
        Lbrace,
        Ident("x".into()),
        Plus,
        Ident("y".into()),
        Semicolon,
        Rbrace,
        Semicolon,
        Let,
        Ident("result".into()),
        Assign,
        Ident("add".into()),
        Lparen,
        Ident("five".into()),
        Comma,
        Ident("ten".into()),
        Rparen,
        Semicolon,
        Bang,
        Minus,
        Slash,
        Asterisk,
        Int(5),
        Semicolon,
        Int(5),
        Lt,
        Int(10),
        Gt,
        Int(5),
        Semicolon,
        If,
        Lparen,
        Int(5),
        Lt,
        Int(10),
        Rparen,
        Lbrace,
        Return,
        True,
        Semicolon,
        Rbrace,
        Else,
        Lbrace,
        Return,
        False,
        Semicolon,
        Rbrace,
        Int(10),
        Eq,
        Int(10),
        Semicolon,
        Int(10),
        NotEq,
        Int(9),
        Semicolon,
    ];

    let lexer = Lexer::new(input);

    for (i, token) in lexer.enumerate() {
        assert_eq!(token, expected[i]);
    }
}
