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
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

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
