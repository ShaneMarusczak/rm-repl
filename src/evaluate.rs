use rusty_maths::equation_analyzer::calculator::calculate;

use crate::{
    repl::{PreviousAnswer, Repl},
    variables,
};

pub(crate) fn evaluate(line: &str, repl: &mut Repl) {
    let line_internal = variables::insert_ans_vars(line, repl);

    let val = calculate(&line_internal);
    if let Ok(v) = val {
        repl.previous_answer(v, true);
        println!("{v:.2}");
    } else {
        repl.previous_answer(0.0, false);
        eprintln!("{}", val.unwrap_err());
    }
}

pub(crate) fn simple_evaluate(line: &str) {
    let val = calculate(line);
    if let Ok(v) = val {
        println!("{v:.2}");
    } else {
        eprintln!("{}", val.unwrap_err());
    }
}
