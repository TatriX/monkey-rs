use env_logger;
use monkey::ast::*;
use monkey::parser::Parser;

fn parse(input: &str) -> Program {
    Parser::parse(input).expect("Parse error")
}

fn assert_integer_literal_expression(expr: Expression, value: i64) {
    match expr {
        Expression::IntegerLiteral(int) => assert_eq!(int, value),
        _ => panic!("expected integer literal, got: {:?}", expr),
    }
}

fn first_statement(program: Program) -> Statement {
    program
        .statements
        .into_iter()
        .next()
        .expect("Program has no statements")
}

#[test]
fn test_let_statements() {
    let _ = env_logger::try_init();

    let input = "
let x = 5;
let y = 10;
let foobar = 838383;
";

    let program = parse(input);

    assert_eq!(program.statements.len(), 3);

    let expected = vec!["x", "y", "foobar"];

    for (i, stmt) in program.statements.iter().enumerate() {
        match stmt {
            Statement::Let(let_stmt) => {
                assert_eq!(let_stmt.name, expected[i]);
            }
            _ => panic!("expected let statement, got {:?}", stmt),
        }
    }
}

#[test]
fn test_return_statements() {
    let _ = env_logger::try_init();

    let input = "
return 5;
return 10;
return 993322;
";
    let program = parse(input);

    assert_eq!(program.statements.len(), 3);

    for (_i, stmt) in program.statements.iter().enumerate() {
        match stmt {
            Statement::Return(_let_stmt) => {}
            _ => panic!("expected let statement, got {:?}", stmt),
        }
    }
}

#[test]
fn test_identifier_expression() {
    let _ = env_logger::try_init();

    let program = parse("foobar;");

    assert_eq!(program.statements.len(), 1);

    match first_statement(program) {
        Statement::Expression(expr) => match &expr.expression {
            Expression::Identifier(name) => assert_eq!(name, "foobar"),
            _ => panic!("expected identifier, got: {:?}", expr),
        },
        got => panic!("expected experssion, got: {:?}", got),
    }
}

#[test]
fn test_integer_literal_expression() {
    let _ = env_logger::try_init();

    let input = "5;";
    let program = parse(input);

    assert_eq!(program.statements.len(), 1);

    match first_statement(program) {
        Statement::Expression(expr) => assert_integer_literal_expression(expr.expression, 5),
        got => panic!("expected experssion, got: {:?}", got),
    }
}

#[test]
fn test_prefix_expression() {
    let _ = env_logger::try_init();

    let tests = vec![
        ("!5", PrefixOperator::Negate, 5),
        ("-15", PrefixOperator::Minus, 15),
    ];

    for (input, operator, value) in tests {
        let program = parse(input);

        assert_eq!(program.statements.len(), 1);

        match first_statement(program) {
            Statement::Expression(expr) => match expr.expression {
                Expression::PrefixExpression(op, right_expr) => {
                    assert_eq!(op, operator);
                    assert_integer_literal_expression(*right_expr, value);
                }
                _ => panic!("expected prefix expression, got: {:?}", expr),
            },
            got => panic!("expected experssion, got: {:?}", got),
        }
    }
}
