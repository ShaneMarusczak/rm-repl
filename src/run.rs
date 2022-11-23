use rusty_maths::equation_analyzer::calculator::calculate;

use crate::repl::{PreviousAnswer, Repl};

pub(crate) fn run(line: &str, repl: &mut Repl) {
    let val = calculate(line);
    if let Ok(v) = val {
        repl.previous_answer(v, true);
        println!("{}", v);
    } else {
        repl.previous_answer(0.0, false);
        eprintln!("{}", val.unwrap_err());
    }
}
