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
    UnexpectedEof,
}

#[allow(unused)] // TODO: remove
#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Precedence {
    Lowest,
    /// ==
    Equals,
    /// > or <
    LessGreater,
    /// +
    Sum,
    /// *
    Product,
    /// -X or !X
    Prefix,
    /// myFunction(X)
    Call,
}

type ParseResult<T> = Result<T, ParseError>;

impl Parser {
    pub fn parse(input: &str) -> ParseResult<Program> {
        Parser::new(Lexer::new(input)).parse_program()
    }

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
            Token::Return => self.parse_return_statement().map(Statement::Return),
            token => self
                .parse_expression_statement(token)
                .map(Statement::Expression),
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
                                name: ident,
                                value: Expression::Identifier("???".into()),
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

    fn parse_return_statement(&mut self) -> ParseResult<ReturnStatement> {
        self.next_token();

        while let Some(token) = self.next_token() {
            if let Token::Semicolon = token {
                return Ok(ReturnStatement {
                    value: Expression::Identifier("???".into()),
                });
            }
        }

        Err(ParseError::UnexpectedEof)
    }

    fn parse_expression_statement(&mut self, token: Token) -> ParseResult<ExpressionStatement> {
        let expression = self.parse_expression(token, Precedence::Lowest)?;

        if let Some(Token::Semicolon) = self.peek_token {
            self.next_token();
        }

        Ok(ExpressionStatement { expression })
    }

    fn parse_expression(
        &mut self,
        token: Token,
        _precedence: Precedence,
    ) -> ParseResult<Expression> {
        match token {
            Token::Ident(value) => Ok(Expression::Identifier(value.clone())),
            Token::Int(int) => Ok(Expression::IntegerLiteral(int)),
            Token::Bang => self.parse_prefix_epxression(PrefixOperator::Negate),
            Token::Minus => self.parse_prefix_epxression(PrefixOperator::Minus),
            token => panic!(
                "Cannot parse prefix expression, unexpected token: {:?}",
                token
            ),
        }
    }

    fn parse_prefix_epxression(&mut self, operator: PrefixOperator) -> ParseResult<Expression> {
        self.next_token();

        self.cur_token
            .take()
            .ok_or(ParseError::UnexpectedEof)
            .and_then(|token| self.parse_expression(token, Precedence::Prefix))
            .map(|right| Expression::PrefixExpression(operator, Box::new(right)))
    }
}
