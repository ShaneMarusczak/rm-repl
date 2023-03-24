use rusty_maths::equation_analyzer::calculator::calculate;

use crate::{
    repl::{PreviousAnswer, Repl},
    variables,
};

pub(crate) fn run(line: &str, repl: &mut Repl) {
    let line_internal = variables::insert_ans_vars(line, repl);

    let val = calculate(&line_internal);
    if let Ok(v) = val {
        repl.previous_answer(v, true);
        println!("{}", v);
    } else {
        repl.previous_answer(0.0, false);
        eprintln!("{}", val.unwrap_err().to_lowercase());
    }
}

pub(crate) fn simple_run(line: &str) {
    let val = calculate(line);
    if let Ok(v) = val {
        println!("{}", v);
    } else {
        eprintln!("{}", val.unwrap_err().to_lowercase());
    }
}
