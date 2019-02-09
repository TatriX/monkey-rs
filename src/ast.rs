use std::fmt::{self, Display};

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for stmt in &self.statements {
            writeln!(f, "{}", stmt)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
}

impl Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Statement::*;

        match &self {
            Let(x) => write!(f, "{}", x),
            Return(x) => write!(f, "{}", x),
            Expression(x) => write!(f, "{}", x),
        }
    }
}

type Identifier = String;

#[derive(Debug)]
pub struct LetStatement {
    pub name: Identifier,
    pub value: Expression,
}

impl Display for LetStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "let {} = {};", self.name, &self.value)
    }
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub value: Expression,
}

impl Display for ReturnStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "return {};", &self.value)
    }
}

// TODO: remove this and use Expression directly?
#[derive(Debug)]
pub struct ExpressionStatement {
    pub expression: Expression,
}

impl Display for ExpressionStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.expression)
    }
}

#[derive(Debug, PartialEq)]
pub enum PrefixOperator {
    Negate,
    Minus,
}

impl Display for PrefixOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PrefixOperator::Negate => "!",
                PrefixOperator::Minus => "-",
            }
        )
    }
}

#[derive(Debug)]
pub enum Expression {
    Identifier(Identifier),
    IntegerLiteral(i64),
    PrefixExpression(PrefixOperator, Box<Expression>),
}

impl Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Expression::*;

        match &self {
            Identifier(x) => write!(f, "{}", x),
            IntegerLiteral(x) => write!(f, "{}", x),
            PrefixExpression(op, right) => write!(f, "({}{})", op, right),
        }
    }
}
