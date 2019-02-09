use env_logger;
use monkey::ast::*;
use monkey::lexer::Lexer;
use monkey::parser::Parser;

#[test]
fn test_let_statements() {
    let _ = env_logger::try_init();

    let input = "
let x = 5;
let y = 10;
let foobar = 838383;
";
    let mut parser = Parser::new(Lexer::new(input));
    let program = parser.parse_program().expect("Parse error");

    assert_eq!(program.statements.len(), 3);

    let expected = vec!["x", "y", "foobar"];

    for (i, stmt) in program.statements.iter().enumerate() {
        match stmt {
            Statement::Let(let_stmt) => {
                assert_eq!(let_stmt.name.value, expected[i]);
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
    let mut parser = Parser::new(Lexer::new(input));
    let program = parser.parse_program().expect("Parse error");

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

    let input = "foobar;";

    let mut parser = Parser::new(Lexer::new(input));
    let program = parser.parse_program().expect("Parse error");

    assert_eq!(program.statements.len(), 1);

    match &program.statements[0] {
        Statement::Expression(expr) => match &expr.expression {
            Expression::Identifier(ident) => assert_eq!(ident.value, "foobar"),
        },
        got => panic!("expected experssion, got: {:?}", got),
    }
}
