use crate::ast::*;
use crate::lexer::Lexer;
use crate::token::Token;

pub struct Parser {
    lexer: Lexer,

    cur_token: Option<Token>,
    peek_token: Option<Token>,
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken { got: Option<Token>, expected: Token },
}

type ParseResult<T> = Result<T, ParseError>;

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Self {
            lexer,
            cur_token: None,
            peek_token: None,
        };
        parser.next_token();
        parser.next_token();
        parser
    }

    fn next_token(&mut self) -> &Option<Token> {
        self.cur_token = self.peek_token.take();
        self.peek_token = self.lexer.next();
        &self.cur_token
    }

    pub fn parse_program(&mut self) -> ParseResult<Program> {
        let mut statements = vec![];

        while let Some(token) = self.cur_token.take() {
            statements.push(self.parse_statement(token)?);
            self.next_token();
        }

        Ok(Program { statements })
    }

    fn parse_statement(&mut self, token: Token) -> ParseResult<Statement> {
        match token {
            Token::Let => self.parse_let_statement().map(Statement::Let),
            _ => unimplemented!("Parsing for {:?} unimplemented", token),
        }
    }

    fn parse_let_statement(&mut self) -> ParseResult<LetStatement> {
        match self.peek_token.take() {
            Some(Token::Ident(ident)) => {
                self.next_token();
                if let Some(Token::Assign) = self.peek_token {
                    while let Some(token) = self.next_token() {
                        if let Token::Semicolon = token {
                            return Ok(LetStatement {
                                name: Identifier { value: ident },
                                value: Expression::Identifier(Identifier {
                                    value: "???".into(),
                                }),
                            });
                        }
                    }
                    Err(ParseError::UnexpectedToken {
                        got: self.peek_token.take(),
                        expected: Token::Semicolon,
                    })
                } else {
                    Err(ParseError::UnexpectedToken {
                        got: self.peek_token.take(),
                        expected: Token::Assign,
                    })
                }
            }
            got => Err(ParseError::UnexpectedToken {
                got,
                expected: Token::Ident("_".into()),
            }),
        }
    }
}
