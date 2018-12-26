use std::io::{stdin, stdout, BufRead, Write};

use monkey::lexer::Lexer;


fn main() {
    repl(stdin().lock(), stdout())
}


fn repl(mut input: impl BufRead, mut output: impl Write) {
    let prompt = ">> ";


    let mut line = String::new();
    loop {
        write!(output, "{}", prompt).expect("Cannot write prompt");
        output.flush().expect("Cannot flush");

        if input.read_line(&mut line).expect("Cannot read line") == 0 {
            break
        }
        let lexer = Lexer::new(&line);

        for (i, token) in lexer.enumerate() {
            writeln!(output, "{:2.}) {:?}", i, token).expect("Cannot write token");
        }
    }
}
